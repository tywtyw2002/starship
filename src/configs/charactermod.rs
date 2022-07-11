use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct CharactermodConfig<'a> {
    pub format: &'a str,
    // pub success_symbol: &'a str,
    // pub error_symbol: &'a str,
    // #[serde(alias = "vicmd_symbol")]
    pub ssh_symbol: &'a str,
    pub normal_symbal: &'a str,
    pub root_symbal: &'a str,
    pub normal_vimcmd_symbal: &'a str,
    pub root_vimcmd_symbal: &'a str,
    pub success_style: &'a str,
    pub root_style: &'a str,
    pub error_style: &'a str,
    pub vimcmd_normal_style: &'a str,
    pub vimcmd_root_style: &'a str,
    pub vimcmd_visual_style: &'a str,
    pub vimcmd_replace_style: &'a str,
    pub vimcmd_replace_one_style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for CharactermodConfig<'a> {
    fn default() -> Self {
        CharactermodConfig {
            format: "$ssh_symbol[$symbol]($style) ",
            ssh_symbol: "⌨ ",
            normal_symbal: "⇒ ",
            root_symbal: "➜ ",
            normal_vimcmd_symbal: "⇐ ",
            root_vimcmd_symbal: "❮❮",
            success_style: "cyan",
            root_style: "yellow",
            error_style: "red",
            vimcmd_normal_style: "green",
            vimcmd_root_style: "red",
            vimcmd_visual_style: "yellow",
            vimcmd_replace_style: "purple",
            vimcmd_replace_one_style: "purple",
            disabled: false,
        }
    }
}
