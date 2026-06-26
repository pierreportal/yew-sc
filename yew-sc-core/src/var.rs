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
