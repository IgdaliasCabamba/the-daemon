use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Programs {
    pub bin: String,
    pub description: String,
    pub args: Vec<String>,
    pub output: bool
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
