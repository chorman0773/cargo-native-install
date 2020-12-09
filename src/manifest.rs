
use std::{collections::HashMap, path::{Path, PathBuf}};

use install_dirs::dirs::InstallDirs;
use serde_derive::Deserialize;

use crate::Options;

#[derive(Deserialize,Debug,PartialEq,Eq,Copy,Clone)]
#[serde(rename_all = "lowercase")]
pub enum TargetType{
    Bin,
    SBin,
    Library,
    Shared,
    Libexec,
    Include,
    Sysconfig,
    Data,
    Doc,
    Man,
    Info,
    Run,
}
impl TargetType{
    pub fn get_install_root<'a>(&self,dirs: &'a InstallDirs,opts: &Options) -> Option<&'a Path>{
        match self{
            TargetType::Bin => Some(&*dirs.bin),
            TargetType::SBin => if opts.no_sbin{Some(&*dirs.bin)}else{ Some(&*dirs.sbin)},
            TargetType::Library => Some(&*dirs.lib),
            TargetType::Shared =>{
                match opts.shared_targets_are_libraries{
                    Some(true) => Some(&*dirs.lib),
                    Some(false) => Some(&*dirs.bin),
                    None if std::env::consts::DLL_EXTENSION=="dll" => Some(&*dirs.bin),
                    None => Some(&*dirs.lib)
                }
            }
            TargetType::Libexec => if opts.no_libexec{Some(&*dirs.bin)}else{ Some(&*dirs.libexec)}
            TargetType::Include => Some(&*dirs.include),
            TargetType::Sysconfig => Some(&*dirs.sysconf),
            TargetType::Data => Some(&*dirs.data),
            TargetType::Doc => Some(&*dirs.doc),
            TargetType::Man => Some(&*dirs.man),
            TargetType::Info => Some(&*dirs.info),
            TargetType::Run => None
        }
    }
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
    pub installed_path: Option<PathBuf>,
    #[serde(default)]
    pub target_file: Option<PathBuf>,
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(default)]
    pub installed_aliases: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub exclude: bool,
    #[serde(default)]
    pub strip: Option<bool>
}

#[derive(Deserialize,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NativeInstallMetadata{
    pub install_targets: HashMap<String,Target>
}
