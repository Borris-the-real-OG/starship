use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct VagrantConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for VagrantConfig<'a> {
    fn default() -> Self {
        VagrantConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "⍱ ",
            style: "cyan bold",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["Vagrantfile"],
            detect_folders: vec![],
        }
    }
}
