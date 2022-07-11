use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct HostnamemodConfig<'a> {
    pub trim_at: &'a str,
    pub format: &'a str,
    pub style_ssh: &'a str,
    pub style_local: &'a str,
    pub disabled: bool,
}

impl<'a> Default for HostnamemodConfig<'a> {
    fn default() -> Self {
        HostnamemodConfig {
            trim_at: ".",
            format: "[$hostname]($style)",
            style_ssh: "purple",
            style_local: "yellow",
            disabled: false,
        }
    }
}
