
use std::{collections::HashMap, path::PathBuf};

use serde_derive::Deserialize;

#[derive(Deserialize,Debug,PartialEq,Eq)]
#[serde(rename_all = "lowercase")]
pub enum TargetType{
    Bin,
    Library,
    Shared,
    Alias,
    Libexec,
    Include,
    Sysconfig,
    Data,
    Doc,
    Man,
    Info,
    Run,
}

#[derive(Deserialize,Debug,Default)]
#[serde(rename_all = "kebab-case")]
pub struct Target{
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<TargetType>,
    #[serde(default)]
    pub privileged: bool,
    #[serde(default)]
    pub directory: bool,
    #[serde(default)]
    pub install_dir: Option<PathBuf>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub alias_target: Option<String>,
    #[serde(default)]
    pub installed_path: Option<PathBuf>,
    #[serde(default)]
    pub target_file: Option<PathBuf>,
    #[serde(default)]
    pub prefix: Option<String>
}

#[derive(Deserialize,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NativeInstallMetadata{
    pub targets: HashMap<String,Target>
}
