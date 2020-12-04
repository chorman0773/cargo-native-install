
use std::{collections::HashMap, fs::{self, Metadata, Permissions, metadata}, path::{Path, PathBuf}};

use install_dirs::dirs::InstallDirs;
use manifest::{Target, TargetType};


#[derive(Default)]
pub struct Options{
    // Programs
    pub install: Option<PathBuf>,
    pub strip: Option<PathBuf>,

    // Paths
    pub prefix: Option<PathBuf>,
    pub exec_prefix: Option<PathBuf>,
    pub bindir: Option<PathBuf>,
    pub sbindir: Option<PathBuf>,
    pub libdir: Option<PathBuf>,
    pub libexecdir: Option<PathBuf>,
    pub datarootdir: Option<PathBuf>,
    pub datadir: Option<PathBuf>,
    pub includedir: Option<PathBuf>,
    pub sysconfdir: Option<PathBuf>,
    pub sharedstatedir: Option<PathBuf>,
    pub localstatedir: Option<PathBuf>,
    pub infodir: Option<PathBuf>,
    pub mandir: Option<PathBuf>,
    pub localedir: Option<PathBuf>,
    pub docdir: Option<PathBuf>,

    pub user_prefix: bool,
    pub dry_run: bool,
    pub manifest_dir: Option<PathBuf>,
    pub mode: Option<String>,
    pub no_create_dirs: bool,
    pub verbose: bool,
    pub force: bool,
    pub install_privileged: Option<bool>,
    pub install_target: Option<String>,
    pub no_libexec: bool,
    pub no_sbin: bool,
    pub build: bool,
    pub no_install: bool,
    pub shared_targets_are_libraries: Option<bool>,
    pub out_dir: Option<PathBuf>,
    pub debug: bool,
}

const VERSION: &str = std::env!("CARGO_PKG_VERSION");

const DEFAULT_TARGET: &str = std::env!("TARGET");

pub fn parse( mut args: std::env::Args) -> Options{
    let mut opts = Options{
        install: which::which("install").ok(),

        strip: which::which("strip").ok(),
        ..Default::default()
    };

    let prg_name = args.next().unwrap();

    for arg in args{
        match &*arg{
            "--help" =>{
                println!("Usage: {} [options]...",&prg_name);
                println!("Installs the current cargo project into native system directories (like GNU make install or cmake --install)");
                println!("Options:");
                println!("\t--help: Prints this message, and exits");
                println!("\t--version: Prints version information, and exits");
                println!("\t--dry-run: Show the results of each install operation, but do not perform any operations");
                println!("\t--user-prefix: Default prefix to ~/.local, instead of a system-wide dir");
                println!("\t--prefix=<prefix>: Sets the prefix for installation operations");
                println!("\t--bindir=<dir>: Use dir as the directory to install binary programs. Either an absolute path, or a path relative to prefix. (defaults to bin)");
                println!("\t--libdir=<dir>: Use dir as the directory to install libraries. Either an absolute path, or a path relative to prefix (defaults to lib)");
                println!("\t--sbindir=<dir>: Use dir as the directory to install system administrator programs. Either an absolute path, or a path relative to prefix (defaults to sbin)");
                println!("\t--libexecdir=<dir>: Use dir as the directory to install programs that aren't for direct use from the shell. Either an absolute path, or a path relative to prefix (defaults to libexec)");
                println!("\t--includedir=<dir>: Use dir as the directory to install header files. Either an absolute path, or a path relative to prefix (defaults to include)");
                println!("\t--datarootdir=<dir>: Use dir as the prefix for platform independent data, documentation, and manuals. Either an absolute path, or a path relative to prefix (defaults to share)");
                println!("\t--datadir=<dir>: Use dir as the directory to install platform independent data. Either an absolute path, or a path relative to the data root (defaults to the same directory as the data root)");
                println!("\t--mandir=<dir>: Use dir as the directory for installing manual pages. Either an absolute path, or a path relative to data root (defaults to man)");
                println!("\t--infodir=<dir>: Use dir as the directory for installing info pages. Either an absolute path, or a path relative to data root (defaults to info)");
                println!("\t--docdir=<dir>: Use dir as the directory for installing project documentation. Either an absolute path, or a path relative to data root (defaults to doc/<project>)");
                println!("\t--localedir=<dir>: Use dir as the directory for installing locale specific information. Either an absolute path, or a path relative to data root (defaults to locale)");
                println!("\t--sysconfdir=<dir>: Use dir as the directory for system configuration files. Either an absolute path, or a path relative to the prefix (defaults to etc)");
                println!("\t--localstatedir=<dir>: Use dir as the directory for local system state. Either an absolute path, or a path relative to the prefix (defaults to var)");
                println!("\t--sharedstatedir=<dir>: Use dir as the directory for shared system state. Either an absolute path, or a path relative to the prefix (defaults to com)");
                println!("\t--manifiest-dir=<dir>: Indicates the directory to the cargo manifest.");
                println!("\t--no-strip: Do not strip programs, even if strip is found");
                println!("\t--without-strip: Same as --no-strip");
                println!("\t--strip=<prg>: Use <prg> to strip, instead of the default (strip)");
                println!("\t--install=<prg>: Use <prg> to install programs, instead of the default (install)");
                println!("\t--internal-install: Do not invoke any programs to install. Instead, copy files natively. This is the default if install is not found, and `--install` is not provided");
                println!("\t--mode=<mode>: Force installed files to use <mode> in the form of a chmod mode (X is the executable bit if the file is a binary target, or a directory)");
                println!("\t--no-create: Do not create installed directories. Also do not create any prefix directories");
                println!("\t--verbose: Print messages for each action");
                println!("\t--force: Install all files, even if this would replace files that are newer");
                println!("\t--no-privileged: Do not install privileged binaries (those installed to sbin)");
                println!("\t--privileged: Install privilged binaries to sbindir, even if a user-specific prefix is used");
                println!("\t--target=<target>: Install only this target");
                println!("\t--no-libexec: Install libexec targets to bin instead");
                println!("\t--no-sbin: Install privileged binaries to bin instead of sbin (note that this does not enable privileged binaries)");
                println!("\t--arch-prefix[=target]: Install bin, lib, include, libexec, sbin targets to an an architecture specific prefix.");
                println!("\t--build: Build the package before installing. An environment variable corresponding to each directory is set during the build");
                println!("\t--build-only: Build the package without installing. Like --build, environment variables will be set with all the directories.");
                println!("\t--shared=lib: Treat cdylib targets as library targets by default and install to libdir. This is the default on unix-like targets");
                println!("\t--shared=bin: Treat cdylib targets as binary targets by default and install to bindir. This is the default on windows");
                println!("\t--out-dir=<dir>: Consider cargo targets to be stored in <dir> instead of <manifest-dir>/target");
                println!("\t--release: Consider cargo targets to have been built in release mode (default)");
                println!("\t--debug: Consider cargo targets to have been built in debug mode");
                std::process::exit(0)
            }
            "--version" =>{
                println!("cargo-native-install v{}",VERSION);
                println!("Copyright (C) 2020 Connor Horman");
                println!("This program is a free software, distributed under the terms of the GNU General Public License, at version 3.0, or (at your option) any later version");
                println!("This program is distributed AS-IS without any waranty.");
                std::process::exit(0)
            }
            "--dry-run" => opts.dry_run = true,
            "--user-prefix" =>opts.user_prefix = true,
            x if x.starts_with("--prefix=") => opts.prefix = x.get(9..).map(Into::into),
            x if x.starts_with("--bindir=") => opts.bindir = x.get(9..).map(Into::into),
            x if x.starts_with("--libdir=") => opts.libdir = x.get(9..).map(Into::into),
            x if x.starts_with("--libexecdir=") => opts.libexecdir = x.get(13..).map(Into::into),
            x if x.starts_with("--includedir=") => opts.includedir = x.get(13..).map(Into::into),
            x if x.starts_with("--sbindir=") => opts.sbindir = x.get(10..).map(Into::into),
            x if x.starts_with("--datarootdir=") => opts.datarootdir = x.get(14..).map(Into::into),
            x if x.starts_with("--datadir=") => opts.datadir = x.get(10..).map(Into::into),
            x if x.starts_with("--mandir=") => opts.mandir = x.get(9..).map(Into::into),
            x if x.starts_with("--infodir=") => opts.infodir = x.get(10..).map(Into::into),
            x if x.starts_with("--docdir=") => opts.docdir = x.get(9..).map(Into::into),
            x if x.starts_with("--localedir=") => opts.localedir = x.get(12..).map(Into::into),
            x if x.starts_with("--localstatedir=") => opts.localstatedir = x.get(16..).map(Into::into),
            x if x.starts_with("--sharedstatedir=") => opts.sharedstatedir = x.get(17..).map(Into::into),
            x if x.starts_with("--sysconfdir=") => opts.sysconfdir = x.get(13..).map(Into::into),
            x if x.starts_with("--manifest-dir=") => opts.manifest_dir = x.get(15..).map(Into::into),
            "--no-create" => opts.no_create_dirs = true,
            "--no-strip" | "--without-strip" => opts.strip = None,
            x if x.starts_with("--strip=") => opts.strip = x.get(8..).map(which::which).map(Result::ok).flatten(),
            x if x.starts_with("--install=") => opts.install = x.get(10..).map(which::which).map(Result::ok).flatten(),
            "--internal-install" => opts.install = None,
            x if x.starts_with("--mode=") => opts.mode = x.get(7..).map(ToOwned::to_owned),
            "--verbose" => opts.verbose = true,
            "--force" => opts.force = true,
            "--no-privileged" => opts.install_privileged = Some(false),
            "--privileged" => opts.install_privileged = Some(true),
            x if x.starts_with("--target=") => opts.install_target = x.get(9..).map(ToOwned::to_owned),
            "--no-libexec" => opts.no_libexec = true,
            "--no-sbin" => opts.no_sbin = true,
            "--arch-target" => opts.exec_prefix = Some(DEFAULT_TARGET.into()),
            x if x.starts_with("--arch-target=") => opts.exec_prefix = x.get(14..).map(Into::into),
            "--build" => opts.build = true,
            "--build-only" => {
                opts.build = true;
                opts.no_install = true;
            },
            "--shared=lib" => opts.shared_targets_are_libraries = Some(true),
            "--shared=bin" => opts.shared_targets_are_libraries = Some(false),
            x if x.starts_with("--out-dir=") => opts.out_dir = x.get(10..).map(Into::into),
            "--debug" => opts.debug = true,
            "--release" => opts.debug = false,
            x => {
                eprintln!("Unrecongized option {}. ",x);
                std::process::exit(1);
            }
        }
    }
    opts
}

mod manifest;


fn main(){
    let opts = parse(std::env::args());
    
    let manifest_dir = if let Some(dir) = &opts.manifest_dir{
        dir.clone()
    }else{
        std::env::current_dir().unwrap()
    };

    let manifest ={
        let mut manifest = PathBuf::new();
        manifest.push(&manifest_dir);
        manifest.push("Cargo.toml");
        manifest
    };

    let manifest = cargo_toml::Manifest::<manifest::NativeInstallMetadata>::from_path_with_metadata(manifest);
    match manifest{
        Ok(manifest) => {
            let mut targets;
            let project_name;
            if let Some(package) = manifest.package{
                project_name = package.name;

                if let Some(metadata) = package.metadata{
                    targets = metadata.targets;
                }else{
                    targets = HashMap::new();
                }
                for product in manifest.bin{
                    let name = if let Some(name) = product.name{
                        name
                    }else{
                        project_name.replace("-", "_")
                    };

                    let target = match targets.get_mut(&name){
                        Some(target) => target,
                        None => {
                            targets.insert(name.clone(), Target{
                                type_: Some(TargetType::Bin),
                                ..Default::default()
                            });
                            targets.get_mut(&name).unwrap()
                        }
                    };

                    if let None = target.mode{
                        target.mode = Some("u=rwx,g=rx,o=rx".to_string());
                    }

                    if let None = target.type_{
                        target.type_ = Some(TargetType::Bin)
                    }
                    
                    if let None = target.installed_path{
                        target.installed_path = Some((&*name).into());
                    }


                    if let Some(buf) = &mut target.installed_path{
                        match std::env::consts::EXE_EXTENSION{
                            "" => (),
                            x => {buf.set_extension(x);}
                        }
                    }

                    let mut target_path = PathBuf::new();
                    if let Some(dir) = &opts.out_dir{
                        target_path.push(dir);
                    }else{
                        target_path.push("target");
                    }

                    if opts.debug{
                        target_path.push("debug");
                    }else{
                        target_path.push("release");
                    }

                    target_path.push(&name);

                    match std::env::consts::EXE_EXTENSION{
                        "" => (),
                        x => {target_path.set_extension(x);}
                    }
                }

                for product in manifest.lib{
                    if product.crate_type.len()!=1{
                        for crate_type in product.crate_type{
                            let name = product.name.as_ref().cloned().unwrap_or( project_name.replace("-", "_"))+"-"+&*crate_type;
                            let target = match targets.get_mut(&name){
                                Some(target) => target,
                                None => {
                                    targets.insert(name.clone(),Target{
                                        type_: Some(
                                            if crate_type=="staticlib"{
                                                TargetType::Library
                                            }else if crate_type=="cdylib"{
                                                TargetType::Shared
                                            }else{
                                                continue
                                            }),
                                        ..Default::default()
                                    });
                                    targets.get_mut(&name).unwrap()
                                }
                            };
                            if let None = target.mode{
                                target.mode = Some("u=rw,g=r,o=r".to_string());
                            }

                            if let None = target.prefix{
                                target.prefix = Some(std::env::consts::DLL_PREFIX.to_string())
                            }

                            if let None = target.installed_path{
                                let mut path = PathBuf::new();
                                let fname = target.prefix.as_ref().cloned().unwrap()+&*name;
                                path.push(&fname);
                                path.set_extension(match &*crate_type{
                                    "dylib" | "cdylib" => std::env::consts::DLL_EXTENSION,
                                    "staticlib" => if cfg!(windows){
                                        "lib"
                                        }else{
                                            "a"
                                        }
                                    "rlib" => ".rlib",
                                    _ => panic!("wut")
                                });
                                target.installed_path =Some(path);
                            }

                            if let None = target.target_file{
                                let mut path = PathBuf::new();
                                let fname = "lib".to_string()+&*name;
                                if let Some(dir) = &opts.out_dir{
                                    path.push(&dir);
                                }else{
                                    path.push(&manifest_dir);
                                    path.push("target");
                                    path.push(if opts.debug{"debug"}else{"release"});
                                }
                                path.push(&fname);
                                path.set_extension(match &*crate_type{
                                    "dylib" | "cdylib" => std::env::consts::DLL_EXTENSION,
                                    "staticlib" => if cfg!(windows){
                                        "lib"
                                        }else{
                                            "a"
                                        }
                                    "rlib" => ".rlib",
                                    _ => panic!("wut")
                                });
                                target.target_file =Some(path);
                            }
                        }
                    }else{
                        let crate_type = product.crate_type[1].clone();
                        let name = product.name.as_ref().cloned().unwrap_or( project_name.replace("-", "_"));
                        let target = match targets.get_mut(&name){
                            Some(target) => target,
                            None => {
                                targets.insert(name.clone(),Target{
                                    type_: Some(
                                        if crate_type=="staticlib"{
                                            TargetType::Library
                                        }else if crate_type=="cdylib"{
                                            TargetType::Shared
                                        }else{
                                            continue
                                        }),
                                    ..Default::default()
                                });
                                targets.get_mut(&name).unwrap()
                            }
                        };
                        if let None = target.mode{
                            target.mode = Some("u=rw,g=r,o=r".to_string());
                        }

                        if let None = target.prefix{
                            target.prefix = Some(std::env::consts::DLL_PREFIX.to_string())
                        }

                        if let None = target.installed_path{
                            let mut path = PathBuf::new();
                            let fname = target.prefix.as_ref().cloned().unwrap()+&*name;
                            path.push(&fname);
                            path.set_extension(match &*crate_type{
                                "dylib" | "cdylib" => std::env::consts::DLL_EXTENSION,
                                "staticlib" => if cfg!(windows){
                                    "lib"
                                    }else{
                                        "a"
                                    }
                                "rlib" => ".rlib",
                                _ => panic!("wut")
                            });
                            target.installed_path =Some(path);
                        }

                        if let None = target.target_file{
                            let mut path = PathBuf::new();
                            let fname = "lib".to_string()+&*name;
                            if let Some(dir) = &opts.out_dir{
                                path.push(&dir);
                            }else{
                                path.push(&manifest_dir);
                                path.push("target");
                                path.push(if opts.debug{"debug"}else{"release"});
                            }
                            path.push(&fname);
                            path.set_extension(match &*crate_type{
                                "dylib" | "cdylib" => std::env::consts::DLL_EXTENSION,
                                "staticlib" => if cfg!(windows){
                                    "lib"
                                    }else{
                                        "a"
                                    }
                                "rlib" => ".rlib",
                                _ => panic!("wut")
                            });
                            target.target_file =Some(path);
                        }
                    }
                }
            }else{
                eprintln!("Cannot install this crate, no packages contained (is this a virtual manifest)");
                std::process::exit(1)
            }
            if opts.build{
                let mut cargo = std::process::Command::new("cargo");
                cargo.arg("build");
                cargo.current_dir(&manifest_dir);
                if let Some(dir) = &opts.out_dir{
                    cargo.arg("--target-dir");
                    cargo.arg(dir);
                }
                if opts.verbose{
                    cargo.arg("--verbose");
                }

                if !opts.debug{
                    cargo.arg("--release");
                }

                if let Some(prefix) = &opts.prefix{
                    cargo.env("prefix",prefix);
                }

                if let Some(exec_prefix) = &opts.exec_prefix{
                    cargo.env("exec_prefix",exec_prefix);
                }

                if let Some(dir) = &opts.bindir{
                    cargo.env("bindir",dir);
                }

                if let Some(dir) = &opts.libdir{
                    cargo.env("libdir",dir);
                }

                if let Some(dir) = &opts.sbindir{
                    cargo.env("sbindir",dir);
                }

                if let Some(dir) = &opts.libexecdir{
                    cargo.env("libexecdir",dir);
                }

                if let Some(dir) = &opts.includedir{
                    cargo.env("includedir",dir);
                }
                
                if let Some(dir) = &opts.datarootdir{
                    cargo.env("dataroot",dir);
                }

                if let Some(dir) = &opts.datadir{
                    cargo.env("datadir",dir);
                }
                if let Some(dir) = &opts.mandir{
                    cargo.env("mandir",dir);
                }

                if let Some(dir) = &opts.docdir{
                    cargo.env("docdir",dir);
                }

                if let Some(dir) = &opts.infodir{
                    cargo.env("infodir",dir);
                }

                if let Some(dir) = &opts.localedir{
                    cargo.env("localedir",dir);
                }

                if let Some(dir) = &opts.localstatedir{
                    cargo.env("localstatedir",dir);
                }

                if let Some(dir) = &opts.sharedstatedir{
                    cargo.env("sharedstatedir",dir);
                }

                if let Some(dir) = &opts.sysconfdir{
                    cargo.env("sysconfdir",dir);
                }

                match cargo.status(){
                    Ok(status) => {
                        if !status.success(){
                            eprintln!("Failed to run cargo, command exited with non-zero code");
                            std::process::exit(1)
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to run cargo, {}",e);
                        std::process::exit(1);
                    }
                }
            }

            if !opts.no_install{
                let mut dirs = InstallDirs::with_project_name(&project_name);

                if let Some(dir) = &opts.prefix{
                    dirs.prefix = dir.clone()
                }

                if let Some(dir) = &opts.exec_prefix{
                    dirs.prefix = dir.clone()
                }

                if let Some(dir) = &opts.bindir{
                    dirs.bin = dir.clone()
                }

                if let Some(dir) = &opts.libdir{
                    dirs.lib = dir.clone()
                }
                if let Some(dir) = &opts.sbindir{
                    dirs.sbin = dir.clone()
                }
                if let Some(dir) = &opts.libexecdir{
                    dirs.libexec = dir.clone()
                }
                if let Some(dir) = &opts.includedir{
                    dirs.include = dir.clone()
                }

                if let Some(dir) = &opts.datarootdir{
                    dirs.dataroot = dir.clone()
                }
                if let Some(dir) = &opts.datadir{
                    dirs.data = dir.clone()
                }
                if let Some(dir) = &opts.mandir{
                    dirs.man = dir.clone()
                }
                if let Some(dir) = &opts.docdir{
                    dirs.doc = dir.clone()
                }
                if let Some(dir) = &opts.infodir{
                    dirs.info = dir.clone()
                }
                if let Some(dir) = &opts.localedir{
                    dirs.locale = dir.clone()
                }

                if let Some(dir) = &opts.sharedstatedir{
                    dirs.sharedstate = dir.clone()
                }
                if let Some(dir) = &opts.localstatedir{
                    dirs.localstate = dir.clone()
                }
                let dirs = match dirs.canonicalize(){
                    Ok(x) => x,
                    Err(e) =>{
                        eprintln!("Failed to resolve installation prefix: {}",e);
                        std::process::exit(1)
                    }
                };

                if let Some(target) = &opts.install_target{
                    match targets.get(target){
                        Some(target) => install_target(&dirs,target,&opts),
                        None => {
                            eprintln!("Cannot install target {}, no such target exists",target);
                            std::process::exit(1)
                        }
                    }
                }else {
                    for target in targets.values(){
                        install_target(&dirs,target,&opts);
                    }
                }

            }
        },
        Err(err) => {
            eprintln!("Failed to parse cargo manifest {}",err);
            std::process::exit(1)
        }
    }
}

pub fn install_target(dirs: &InstallDirs,target: &Target,opts: &Options){
    
}

pub fn create_alias<P1: AsRef<Path>,P2: AsRef<Path>>(src: P1,dest: P2,opts: &Options) -> std::io::Result<()>{
    if !opts.dry_run{
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(src,dest)
        }
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_file(src,dest)
        }
        #[cfg(not(any(unix,windows)))]
        {
            panic!("Unsupported operating system for alias targets")
        }
    }else{
        Ok(())
    }
}

pub fn do_internal_install<P1: AsRef<Path>,P2: AsRef<Path>>(src: P1,dest: P2,opts: &Options,target: &Target) -> std::io::Result<()>{
    if !opts.dry_run{
        if !opts.force{
            let src_md = metadata(src.as_ref());
            let dest_md = metadata(dest.as_ref());
            match (src_md.and_then(|m|m.modified()),dest_md.and_then(|m|m.modified())){
                (Ok(src_time),Ok(dest_time)) =>{
                    if src_time < dest_time{
                        return Ok(());
                    }
                },
                (_,_) =>()
            }
        }
        fs::copy(src,dest.as_ref())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let dest_permissions = metadata(dest.as_ref())?.permissions();
            if let Some(mode) = &opts.mode{
                let umask = unsafe{libc::umask(0)};
                let mode = if mode.starts_with(|c: char|c.is_digit(8)){
                    u32::from_str_radix(&*mode,8).unwrap() & !umask
                }else if mode.starts_with('=') && mode[1..].starts_with(|c: char|c.is_digit(8)){
                    u32::from_str_radix(&mode[1..],8).unwrap()
                }else if mode.starts_with('+') && mode[1..].starts_with(|c: char|c.is_digit(8)){
                    u32::from_str_radix(&mode[1..],8).unwrap() | dest_permissions.mode()
                }else if mode.starts_with('-') && mode[1..].starts_with(|c: char|c.is_digit(8)){
                    dest_permissions.mode() & !u32::from_str_radix(&mode[1..],8).unwrap()
                }else{
                    let mut mode_bits = dest_permissions.mode();
                    for s in mode.split(","){
                        let mut chars = s.chars();
                        let mut type_mask = 0;
                        let mut cmode = 0;
                        let mut modifier = ' '; // Not valid
                        while let Some(c) = chars.next(){
                            if c=='='||c=='+'||c=='-'{
                                modifier = c;
                                break;
                            }
                            match c{
                                'u' => type_mask |= 0o4700,
                                'g' => type_mask |= 0o2070,
                                'o' => type_mask |= 0o1007,
                                'a' => type_mask |= 0o7777,
                                _ =>{
                                    eprintln!("Invalid mode {}",mode);
                                    std::process::exit(1)
                                }
                            }
                            if type_mask==0{
                                type_mask = 0o7777 & !umask;
                            }
                        }
                        for c in chars{
                            match c{
                                'r' => cmode |= 0o444,
                                'w' => cmode |= 0o222,
                                'x' => cmode |= 0o111,
                                'X' => cmode |=  if mode_bits&0o111!=0 || target.type_ == Some(TargetType::Bin) || target.directory{0o111}else{0},
                                's' => cmode |= 0o6000,
                                't' => cmode |= 0o1000,
                                _ => {
                                    eprintln!("Invalid mode {}",mode);
                                    std::process::exit(1)
                                }
                            }
                        }

                        match modifier{
                            '=' => mode_bits = cmode&type_mask | mode_bits&0o2000,
                            '+' => mode_bits |=cmode&type_mask,
                            '-' => mode_bits &= !(cmode&type_mask),
                            _ => {
                                eprintln!("Invalid mode {}",mode);
                                std::process::exit(1)
                            }
                        }
                    }
                    mode_bits
                };
                std::fs::set_permissions(dest.as_ref(), Permissions::from_mode(mode))?;
            }
        }
        
        Ok(())
    }else{
        Ok(())
    }
}
