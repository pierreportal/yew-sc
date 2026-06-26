use yew::AttrValue;

pub trait ToCssVar {
    fn css_var_value(&self) -> Option<String>;
}

impl ToCssVar for AttrValue {
    fn css_var_value(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl ToCssVar for String {
    fn css_var_value(&self) -> Option<String> {
        Some(self.clone())
    }
}

impl ToCssVar for &str {
    fn css_var_value(&self) -> Option<String> {
        Some((*self).to_string())
    }
}

impl ToCssVar for bool {
    fn css_var_value(&self) -> Option<String> {
        Some(self.to_string())
    }
}

macro_rules! impl_to_css_var_display {
    ($($t:ty),*) => {
        $(
            impl ToCssVar for $t {
                fn css_var_value(&self) -> Option<String> {
                    Some(self.to_string())
                }
            }
        )*
    };
}

impl_to_css_var_display!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

impl<T: ToCssVar> ToCssVar for Option<T> {
    fn css_var_value(&self) -> Option<String> {
        self.as_ref().and_then(ToCssVar::css_var_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attr_value_yields_string() {
        let v: AttrValue = AttrValue::from("hello");
        assert_eq!(v.css_var_value().as_deref(), Some("hello"));
    }

    #[test]
    fn string_yields_clone() {
        let v: String = "world".to_string();
        assert_eq!(v.css_var_value().as_deref(), Some("world"));
    }

    #[test]
    fn str_yields_owned_string() {
        let v: &str = "static";
        assert_eq!(v.css_var_value().as_deref(), Some("static"));
    }

    #[test]
    fn bool_yields_true_false() {
        assert_eq!(true.css_var_value().as_deref(), Some("true"));
        assert_eq!(false.css_var_value().as_deref(), Some("false"));
    }

    #[test]
    fn integers_render_decimal() {
        assert_eq!(42i32.css_var_value().as_deref(), Some("42"));
        assert_eq!((-7i64).css_var_value().as_deref(), Some("-7"));
        assert_eq!(0u8.css_var_value().as_deref(), Some("0"));
        assert!(usize::MAX.css_var_value().is_some());
    }

    #[test]
    fn floats_render_with_dot() {
        assert_eq!(1.5f32.css_var_value().as_deref(), Some("1.5"));
        assert_eq!(0.0f64.css_var_value().as_deref(), Some("0"));
    }

    #[test]
    fn option_some_unwraps() {
        let v: Option<&str> = Some("yes");
        assert_eq!(v.css_var_value().as_deref(), Some("yes"));
    }

    #[test]
    fn option_none_yields_none() {
        let v: Option<&str> = None;
        assert_eq!(v.css_var_value(), None);
    }

    #[test]
    fn nested_option_none_yields_none() {
        let v: Option<Option<&str>> = Some(None);
        assert_eq!(v.css_var_value(), None);
    }

    #[test]
    fn nested_option_some_unwraps() {
        let v: Option<Option<&str>> = Some(Some("deep"));
        assert_eq!(v.css_var_value().as_deref(), Some("deep"));
    }
}
