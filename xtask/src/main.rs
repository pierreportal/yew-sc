use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use wasmparser::{Parser, Payload};

const SECTION: &str = "yew_sc_css";

fn usage() -> ! {
    eprintln!(
        "usage: yew-sc-extract <input.wasm> <output.css>\n\n\
         Reads CSS payloads embedded in the `{SECTION}` custom section of a\n\
         yew-sc wasm binary (compiled with `--features static-extract`) and\n\
         writes a single deduplicated stylesheet."
    );
    std::process::exit(2);
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(input) = args.next() else { usage() };
    let Some(output) = args.next() else { usage() };
    let input = PathBuf::from(input);
    let output = PathBuf::from(output);

    let bytes = match fs::read(&input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error reading {}: {e}", input.display());
            return ExitCode::from(1);
        }
    };

    let css = match extract(&bytes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error parsing {}: {e}", input.display());
            return ExitCode::from(1);
        }
    };

    if let Some(parent) = output.parent()
        && !parent.as_os_str().is_empty()
        && let Err(e) = fs::create_dir_all(parent)
    {
        eprintln!("error creating {}: {e}", parent.display());
        return ExitCode::from(1);
    }
    if let Err(e) = fs::write(&output, css.as_bytes()) {
        eprintln!("error writing {}: {e}", output.display());
        return ExitCode::from(1);
    }

    println!(
        "wrote {} ({} bytes) from {}",
        output.display(),
        css.len(),
        input.display()
    );
    ExitCode::SUCCESS
}

fn extract(wasm: &[u8]) -> Result<String, String> {
    // Each entry is `[len:u32 LE][css bytes]`. The linker concatenates all
    // `#[link_section = "yew_sc_css"]` statics, so the section is just a
    // run of entries back to back.
    let mut seen: BTreeSet<String> = BTreeSet::new();

    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload.map_err(|e| format!("invalid wasm: {e}"))?;
        if let Payload::CustomSection(reader) = payload {
            if reader.name() != SECTION {
                continue;
            }
            let mut data = reader.data();
            while !data.is_empty() {
                if data.len() < 4 {
                    return Err(format!("truncated entry header in `{SECTION}`"));
                }
                let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
                data = &data[4..];
                if data.len() < len {
                    return Err(format!("truncated entry body in `{SECTION}` (want {len})"));
                }
                let css = std::str::from_utf8(&data[..len])
                    .map_err(|e| format!("entry is not utf-8: {e}"))?;
                seen.insert(css.to_string());
                data = &data[len..];
            }
        }
    }

    let mut out = String::new();
    for entry in &seen {
        out.push_str(entry);
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_uleb128(out: &mut Vec<u8>, mut value: u32) {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            out.push(byte);
            if value == 0 {
                break;
            }
        }
    }

    /// Build a minimal valid wasm module containing one or more custom
    /// sections whose names and payloads are taken from `sections`.
    fn build_wasm(sections: &[(&str, &[u8])]) -> Vec<u8> {
        let mut wasm = Vec::new();
        wasm.extend_from_slice(b"\0asm");
        wasm.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
        for (name, data) in sections {
            let mut name_bytes = Vec::new();
            write_uleb128(&mut name_bytes, name.len() as u32);
            name_bytes.extend_from_slice(name.as_bytes());

            let mut payload = name_bytes;
            payload.extend_from_slice(data);

            wasm.push(0); // custom section id
            write_uleb128(&mut wasm, payload.len() as u32);
            wasm.extend_from_slice(&payload);
        }
        wasm
    }

    fn entry(css: &str) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + css.len());
        out.extend_from_slice(&(css.len() as u32).to_le_bytes());
        out.extend_from_slice(css.as_bytes());
        out
    }

    #[test]
    fn extracts_single_entry() {
        let mut data = Vec::new();
        data.extend_from_slice(&entry(".a { color: red; }"));
        let wasm = build_wasm(&[(SECTION, &data)]);
        let css = extract(&wasm).unwrap();
        assert_eq!(css, ".a { color: red; }\n");
    }

    #[test]
    fn extracts_multiple_entries_in_sorted_order() {
        let mut data = Vec::new();
        data.extend_from_slice(&entry(".b { color: blue; }"));
        data.extend_from_slice(&entry(".a { color: red; }"));
        let wasm = build_wasm(&[(SECTION, &data)]);
        let css = extract(&wasm).unwrap();
        // BTreeSet sorts lexicographically.
        assert_eq!(css, ".a { color: red; }\n.b { color: blue; }\n");
    }

    #[test]
    fn dedupes_identical_entries() {
        let mut data = Vec::new();
        data.extend_from_slice(&entry(".x { color: red; }"));
        data.extend_from_slice(&entry(".x { color: red; }"));
        let wasm = build_wasm(&[(SECTION, &data)]);
        let css = extract(&wasm).unwrap();
        assert_eq!(css, ".x { color: red; }\n");
    }

    #[test]
    fn ignores_other_custom_sections() {
        let mut data = Vec::new();
        data.extend_from_slice(&entry(".a { color: red; }"));
        let wasm = build_wasm(&[
            ("name", b"\x00\x05hello"),
            (SECTION, &data),
            ("producers", b"\x00"),
        ]);
        let css = extract(&wasm).unwrap();
        assert_eq!(css, ".a { color: red; }\n");
    }

    #[test]
    fn merges_entries_across_multiple_matching_sections() {
        let mut a = Vec::new();
        a.extend_from_slice(&entry(".a { color: red; }"));
        let mut b = Vec::new();
        b.extend_from_slice(&entry(".b { color: blue; }"));
        let wasm = build_wasm(&[(SECTION, &a), (SECTION, &b)]);
        let css = extract(&wasm).unwrap();
        assert_eq!(css, ".a { color: red; }\n.b { color: blue; }\n");
    }

    #[test]
    fn missing_section_yields_empty_output() {
        let wasm = build_wasm(&[]);
        let css = extract(&wasm).unwrap();
        assert_eq!(css, "");
    }

    #[test]
    fn truncated_header_is_an_error() {
        // 3 bytes — shorter than the 4-byte length prefix.
        let wasm = build_wasm(&[(SECTION, &[0x00, 0x00, 0x00])]);
        let err = extract(&wasm).unwrap_err();
        assert!(err.contains("truncated entry header"));
    }

    #[test]
    fn truncated_body_is_an_error() {
        // Claims 10 bytes but only provides 3.
        let mut data = (10u32).to_le_bytes().to_vec();
        data.extend_from_slice(&[1, 2, 3]);
        let wasm = build_wasm(&[(SECTION, &data)]);
        let err = extract(&wasm).unwrap_err();
        assert!(err.contains("truncated entry body"));
    }

    #[test]
    fn non_utf8_entry_is_an_error() {
        let mut data = Vec::new();
        let bad: [u8; 2] = [0xff, 0xfe];
        data.extend_from_slice(&(bad.len() as u32).to_le_bytes());
        data.extend_from_slice(&bad);
        let wasm = build_wasm(&[(SECTION, &data)]);
        let err = extract(&wasm).unwrap_err();
        assert!(err.contains("not utf-8"));
    }

    #[test]
    fn invalid_wasm_is_an_error() {
        let err = extract(b"not a wasm module").unwrap_err();
        assert!(err.contains("invalid wasm"));
    }
}
