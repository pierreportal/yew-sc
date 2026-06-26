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

    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("error creating {}: {e}", parent.display());
                return ExitCode::from(1);
            }
        }
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
