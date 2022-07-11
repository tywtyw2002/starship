use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct GitDirtyConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for GitDirtyConfig<'a> {
    fn default() -> Self {
        GitDirtyConfig {
            format: "$symbol",
            symbol: "[⚡](yellow)",
            disabled: false,
        }
    }
}
