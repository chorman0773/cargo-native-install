use std::{
    collections::HashMap,
    error::Error,
    ffi::{CStr, OsStr, OsString},
    fmt::Display,
    fs::{self, metadata, Permissions},
    io::{ErrorKind, Read},
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};

#[cfg(unix)]
use std::os::unix::prelude::*;

use cargo_toml::Manifest;
use install_dirs::dirs::InstallDirs;
use manifest::{NativeInstallMetadata, Target, TargetType};

#[derive(Default)]
pub struct Options {
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
    pub config: Option<PathBuf>,
}

const VERSION: &str = std::env!("CARGO_PKG_VERSION");

const DEFAULT_TARGET: &str = std::env!("TARGET");

pub fn parse(mut args: std::env::Args) -> Options {
    let mut opts = Options {
        install: which::which("install").ok(),

        strip: which::which("strip").ok(),
        ..Default::default()
    };

    let prg_name = args.next().unwrap();

    for arg in args {
        match &*arg {
            "--help" => {
                println!("Usage: {} [options]...", &prg_name);
                println!("Installs the current cargo project into native system directories (like GNU make install or cmake --install)\n");
                println!("Options:");
                println!("\t--help: Prints this message, and exits");
                println!("\t--version: Prints version information, and exits");
                println!("\t--dry-run: Show the results of each install operation, but do not perform any operations");
                println!(
                    "\t--user-prefix: Default prefix to ~/.local, instead of a system-wide dir. This overrides the `prefix` environment variable even if `--prefix` is not set."
                );
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
                println!("\t--mode=<mode>: Force installed files to use <mode> in the form of a chmod mode (X is the executable bit if the file is a binary target, or a directory). Only guaranteed to be effective on unix-like targets");
                println!("\t--no-create: Do not create installed directories. Also do not create any prefix directories");
                println!("\t--verbose: Print messages for each action");
                println!(
                    "\t--force: Install all files, even if this would replace files that are newer"
                );
                println!("\t--no-privileged: Do not install privileged binaries (those installed to sbin)");
                println!("\t--privileged: Install privilged binaries to sbindir, even if a user-specific prefix is used");
                println!("\t--target=<target>: Install only this target");
                println!("\t--no-libexec: Install libexec targets to bin instead");
                println!("\t--no-sbin: Install privileged binaries to bin instead of sbin (note that this does not enable privileged binaries)");
                println!("\t--arch-prefix[=target]: Install bin, lib, include, libexec, sbin targets to an an architecture specific prefix.");
                println!("\t--build: Build the package before installing. An environment variable corresponding to each directory is set during the build. This option is deprecated");
                println!("\t--build-only: Build the package without installing. Like --build, environment variables will be set with all the directories. This option is deprecated");
                println!("\t--shared=lib: Treat cdylib targets as library targets by default and install to libdir. This is the default on unix-like targets");
                println!("\t--shared=bin: Treat cdylib targets as binary targets by default and install to bindir. This is the default on windows");
                println!("\t--out-dir=<dir>: Consider cargo targets to be stored in <dir> instead of <manifest-dir>/target");
                println!("\t--release: Consider cargo targets to have been built in release mode (default)");
                println!("\t--debug: Consider cargo targets to have been built in debug mode");
                println!("\t--config=<file>: Parse user configuration from the specified file if it exists, rather than config.toml");
                println!("");
                println!("Environment:");
                println!("prefix\n\t\tInstall directories may be specified as environment variables, as well as with options. If both the environment variable and the CLI option is present, the option takes precedence");
                println!("exec_prefix\n\t\t Similar to prefix.");
                println!("bindir\n\t\tSimilar to prefix.");
                println!("libdir\n\t\tSimilar to prefix.");
                println!("sbindir\n\t\tSimilar to prefix.");
                println!("libexecdir\n\t\tSimilar to prefix.");
                println!("includedir\n\t\tSimilar to prefix.");
                println!("datarootdir\n\t\tSimilar to prefix.");
                println!("datadir\n\t\tSimilar to prefix.");
                println!("docdir\n\t\tSimilar to prefix.");
                println!("mandir\n\t\tSimilar to prefix.");
                println!("infodir\n\t\tSimilar to prefix.");
                println!("localedir\n\t\tSimilar to prefix.");
                println!("localstatedir\n\t\tSimilar to prefix.");
                println!("sharedstatedir\n\t\tSimilar to prefix.");
                println!("sysconfdir\n\t\tSimilar to prefix.");
                println!("runstatedir\n\t\tIf specified, the variable is propagate to run targets, and to cargo. Has no further effect on the program");
                println!("PATH\n\t\tSearches for install and strip in these paths");

                std::process::exit(0)
            }
            "--version" => {
                println!("cargo-native-install v{}", VERSION);
                println!("Copyright (C) 2020 Connor Horman");
                println!("This program is a free software, distributed under the terms of the GNU General Public License, at version 3.0, or (at your option) any later version");
                println!("This program is distributed AS-IS without any waranty.");
                std::process::exit(0)
            }
            "--dry-run" => opts.dry_run = true,
            "--user-prefix" => opts.user_prefix = true,
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
            x if x.starts_with("--localstatedir=") => {
                opts.localstatedir = x.get(16..).map(Into::into)
            }
            x if x.starts_with("--sharedstatedir=") => {
                opts.sharedstatedir = x.get(17..).map(Into::into)
            }
            x if x.starts_with("--sysconfdir=") => opts.sysconfdir = x.get(13..).map(Into::into),
            x if x.starts_with("--manifest-dir=") => {
                opts.manifest_dir = x.get(15..).map(Into::into)
            }
            "--no-create" => opts.no_create_dirs = true,
            "--no-strip" | "--without-strip" => opts.strip = None,
            x if x.starts_with("--strip=") => {
                opts.strip = x.get(8..).map(which::which).map(Result::ok).flatten()
            }
            x if x.starts_with("--install=") => {
                opts.install = x.get(10..).map(which::which).map(Result::ok).flatten()
            }
            "--internal-install" => opts.install = None,
            x if x.starts_with("--mode=") => opts.mode = x.get(7..).map(ToOwned::to_owned),
            "--verbose" => opts.verbose = true,
            "--force" => opts.force = true,
            "--no-privileged" => opts.install_privileged = Some(false),
            "--privileged" => opts.install_privileged = Some(true),
            x if x.starts_with("--target=") => {
                opts.install_target = x.get(9..).map(ToOwned::to_owned)
            }
            "--no-libexec" => opts.no_libexec = true,
            "--no-sbin" => opts.no_sbin = true,
            "--arch-target" => opts.exec_prefix = Some(DEFAULT_TARGET.into()),
            x if x.starts_with("--arch-target=") => opts.exec_prefix = x.get(14..).map(Into::into),
            "--build" => opts.build = true,
            "--build-only" => {
                opts.build = true;
                opts.no_install = true;
            }
            "--shared=lib" => opts.shared_targets_are_libraries = Some(true),
            "--shared=bin" => opts.shared_targets_are_libraries = Some(false),
            x if x.starts_with("--out-dir=") => opts.out_dir = x.get(10..).map(Into::into),
            "--debug" => opts.debug = true,
            "--release" => opts.debug = false,
            x if x.starts_with("--config=") => opts.config = x.get(10..).map(Into::into),
            "native-install" => {}
            x => {
                eprintln!("cargo-native-install: Unrecongized option {}. ", x);
                std::process::exit(1);
            }
        }
    }

    if opts.user_prefix {
        if let None = opts.prefix {
            opts.prefix = home::home_dir().map(|mut x| {
                x.push(".local");
                x
            });
        }
    }

    opts
}

mod manifest;

fn get_auto_targets(
    manifest: &mut Manifest<NativeInstallMetadata>,
    targets: &mut HashMap<String, Target>,
    opts: &Options,
    manifest_dir: &Path,
) {
    let project_name;
    if let Some(package) = &mut manifest.package {
        project_name = package.name.clone();

        if let Some(metadata) = &mut package.metadata {
            for (k, v) in metadata.install_targets.drain() {
                targets.insert(k, v);
            }
        }
    } else {
        return;
    }
    for product in &manifest.bin {
        let name = if let Some(name) = &product.name {
            name.clone()
        } else {
            project_name.replace("-", "_")
        };

        let target = match targets.get_mut(&name) {
            Some(target) => target,
            None => {
                targets.insert(
                    name.clone(),
                    Target {
                        type_: Some(TargetType::Bin),
                        ..Default::default()
                    },
                );
                targets.get_mut(&name).unwrap()
            }
        };

        if target.exclude {
            continue;
        }

        if let None = target.mode {
            target.mode = Some("u=rwx,g=rx,o=rx".to_string());
        }

        if let None = target.type_ {
            target.type_ = Some(if target.privileged {
                TargetType::SBin
            } else {
                TargetType::Bin
            })
        }

        if let None = target.installed_path {
            target.installed_path = Some((&*name).into());
        }

        if let None = target.strip {
            target.strip = Some(true)
        }

        if let Some(buf) = &mut target.installed_path {
            match std::env::consts::EXE_EXTENSION {
                "" => (),
                x => {
                    buf.set_extension(x);
                }
            }
        }

        let mut target_path = PathBuf::new();
        if let Some(dir) = &opts.out_dir {
            target_path.push(dir);
        } else {
            target_path.push("target");
        }

        if opts.debug {
            target_path.push("debug");
        } else {
            target_path.push("release");
        }

        target_path.push(&name);

        match std::env::consts::EXE_EXTENSION {
            "" => (),
            x => {
                target_path.set_extension(x);
            }
        }

        target.target_file = Some(target_path)
    }

    for product in &manifest.lib {
        if product.crate_type.len() != 1 {
            for crate_type in &product.crate_type {
                let name = product
                    .name
                    .as_ref()
                    .cloned()
                    .unwrap_or(project_name.replace("-", "_"))
                    + "-"
                    + &*crate_type;
                let target = match targets.get_mut(&name) {
                    Some(target) => target,
                    None => {
                        targets.insert(
                            name.clone(),
                            Target {
                                type_: Some(if crate_type == "staticlib" {
                                    TargetType::Library
                                } else if crate_type == "cdylib" {
                                    TargetType::Shared
                                } else {
                                    continue;
                                }),
                                ..Default::default()
                            },
                        );
                        targets.get_mut(&name).unwrap()
                    }
                };
                if let None = target.mode {
                    target.mode = Some("u=rw,g=r,o=r".to_string());
                }

                if let None = target.strip {
                    target.strip = Some(crate_type == "cdylib");
                }

                if let None = target.prefix {
                    target.prefix = Some(std::env::consts::DLL_PREFIX.to_string())
                }

                if let None = target.installed_path {
                    let mut path = PathBuf::new();
                    let fname = target.prefix.as_ref().cloned().unwrap() + &*name;
                    path.push(&fname);
                    path.set_extension(match &**crate_type {
                        "dylib" | "cdylib" => std::env::consts::DLL_EXTENSION,
                        "staticlib" => {
                            if cfg!(windows) {
                                "lib"
                            } else {
                                "a"
                            }
                        }
                        "rlib" => ".rlib",
                        _ => panic!("wut"),
                    });
                    target.installed_path = Some(path);
                }

                if let None = target.target_file {
                    let mut path = PathBuf::new();
                    let fname = "lib".to_string() + &*name;
                    if let Some(dir) = &opts.out_dir {
                        path.push(&dir);
                    } else {
                        path.push(&manifest_dir);
                        path.push("target");
                        path.push(if opts.debug { "debug" } else { "release" });
                    }
                    path.push(&fname);
                    path.set_extension(match &**crate_type {
                        "dylib" | "cdylib" => std::env::consts::DLL_EXTENSION,
                        "staticlib" => {
                            if cfg!(windows) {
                                "lib"
                            } else {
                                "a"
                            }
                        }
                        "rlib" => ".rlib",
                        _ => panic!("wut"),
                    });
                    target.target_file = Some(path);
                }
            }
        } else {
            let crate_type = product.crate_type[0].clone();
            let name = product
                .name
                .as_ref()
                .cloned()
                .unwrap_or(project_name.replace("-", "_"));
            let target = match targets.get_mut(&name) {
                Some(target) => target,
                None => {
                    targets.insert(
                        name.clone(),
                        Target {
                            type_: Some(if crate_type == "staticlib" {
                                TargetType::Library
                            } else if crate_type == "cdylib" {
                                TargetType::Shared
                            } else {
                                continue;
                            }),
                            ..Default::default()
                        },
                    );
                    targets.get_mut(&name).unwrap()
                }
            };
            if let None = target.mode {
                target.mode = Some("u=rw,g=r,o=r".to_string());
            }

            if let None = target.prefix {
                target.prefix = Some(std::env::consts::DLL_PREFIX.to_string())
            }

            if let None = target.installed_path {
                let mut path = PathBuf::new();
                let fname = target.prefix.as_ref().cloned().unwrap() + &*name;
                path.push(&fname);
                path.set_extension(match &*crate_type {
                    "dylib" | "cdylib" => std::env::consts::DLL_EXTENSION,
                    "staticlib" => {
                        if cfg!(windows) {
                            "lib"
                        } else {
                            "a"
                        }
                    }
                    "rlib" => ".rlib",
                    _ => panic!("wut"),
                });
                target.installed_path = Some(path);
            }

            if let None = target.target_file {
                let mut path = PathBuf::new();
                let fname = "lib".to_string() + &*name;
                if let Some(dir) = &opts.out_dir {
                    path.push(&dir);
                } else {
                    path.push(&manifest_dir);
                    path.push("target");
                    path.push(if opts.debug { "debug" } else { "release" });
                }
                path.push(&fname);
                path.set_extension(match &*crate_type {
                    "dylib" | "cdylib" => std::env::consts::DLL_EXTENSION,
                    "staticlib" => {
                        if cfg!(windows) {
                            "lib"
                        } else {
                            "a"
                        }
                    }
                    "rlib" => ".rlib",
                    _ => panic!("wut"),
                });
                target.target_file = Some(path);
            }
        }
    }
}

fn main() {
    let opts = parse(std::env::args());

    let manifest_dir = if let Some(dir) = &opts.manifest_dir {
        dir.clone()
    } else {
        std::env::current_dir().unwrap()
    };

    let manifest = {
        let mut manifest = PathBuf::new();
        manifest.push(&manifest_dir);
        manifest.push("Cargo.toml");
        manifest
    };

    let manifest = cargo_toml::Manifest::from_path_with_metadata(manifest);
    match manifest {
        Ok(mut manifest) => {
            let mut targets = HashMap::new();
            get_auto_targets(&mut manifest, &mut targets, &opts, &*manifest_dir);

            if let Some(workspace) = &manifest.workspace {
                for member in &workspace.members {
                    let member_dir = {
                        let mut member_dir = manifest_dir.clone();
                        for component in member.split(&['/', '\\'] as &[_]) {
                            member_dir.push(component);
                        }
                        member_dir
                    };
                    let manifest = {
                        let mut manifest = member_dir.clone();
                        manifest.push("Cargo.toml");
                        manifest
                    };
                    let manifest = Manifest::from_path_with_metadata(manifest);
                    if let Ok(mut manifest) = manifest {
                        get_auto_targets(&mut manifest, &mut targets, &opts, &*manifest_dir);
                    }
                }
            }

            let mut dirs;

            let cfg = opts
                .config
                .as_deref()
                .unwrap_or_else(|| Path::new("config.toml"));

            if let Ok(mut f) = std::fs::File::open(cfg) {
                let mut str = String::new();
                if let Err(e) = f.read_to_string(&mut str) {
                    eprintln!(
                        "Failed to read config file {}: {}",
                        cfg.as_os_str().to_str().unwrap_or("Non UTF-8"),
                        e
                    );
                    std::process::exit(1);
                }
                let config: manifest::Config = match toml::from_str(&str) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!(
                            "Failed to read config file {}: {}",
                            cfg.as_os_str().to_str().unwrap_or("Non UTF-8"),
                            e
                        );
                        std::process::exit(1)
                    }
                };

                dirs = config.dirs;
            } else {
                dirs = InstallDirs::defaults();
            }

            dirs.read_env();

            if let Some(dir) = &opts.prefix {
                dirs.prefix = dir.clone()
            }

            if let Some(dir) = &opts.exec_prefix {
                dirs.prefix = dir.clone()
            }

            if let Some(dir) = &opts.bindir {
                dirs.bindir = dir.clone()
            }

            if let Some(dir) = &opts.libdir {
                dirs.libdir = dir.clone()
            }
            if let Some(dir) = &opts.sbindir {
                dirs.sbindir = dir.clone()
            }
            if let Some(dir) = &opts.libexecdir {
                dirs.libexecdir = dir.clone()
            }
            if let Some(dir) = &opts.includedir {
                dirs.includedir = dir.clone()
            }

            if let Some(dir) = &opts.datarootdir {
                dirs.datarootdir = dir.clone()
            }
            if let Some(dir) = &opts.datadir {
                dirs.datadir = dir.clone()
            }
            if let Some(dir) = &opts.mandir {
                dirs.mandir = dir.clone()
            }
            if let Some(dir) = &opts.docdir {
                dirs.docdir = dir.clone()
            }
            if let Some(dir) = &opts.infodir {
                dirs.infodir = dir.clone()
            }
            if let Some(dir) = &opts.localedir {
                dirs.localedir = dir.clone()
            }

            if let Some(dir) = &opts.sharedstatedir {
                dirs.sharedstatedir = dir.clone()
            }
            if let Some(dir) = &opts.localstatedir {
                dirs.localstatedir = dir.clone()
            }
            let dirs = match dirs.canonicalize() {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("Failed to resolve installation prefix: {}", e);
                    std::process::exit(1)
                }
            };

            if opts.build {
                let mut cargo = std::process::Command::new("cargo");
                cargo.arg("build");
                cargo.current_dir(&manifest_dir);
                if let Some(dir) = &opts.out_dir {
                    cargo.arg("--target-dir");
                    cargo.arg(dir);
                }
                if opts.verbose {
                    cargo.arg("--verbose");
                }

                if !opts.debug {
                    cargo.arg("--release");
                }

                cargo.envs(dirs.as_env());

                match cargo.status() {
                    Ok(status) => {
                        if !status.success() {
                            eprintln!("Failed to run cargo, command exited with non-zero code");
                            std::process::exit(1)
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to run cargo, {}", e);
                        std::process::exit(1);
                    }
                }
            }

            if !opts.no_install {
                if let Some(target) = &opts.install_target {
                    match targets.get(target) {
                        Some(target) => install_target(&dirs, target, &opts),
                        None => {
                            eprintln!("Cannot install target {}, no such target exists", target);
                            std::process::exit(1)
                        }
                    }
                } else {
                    for target in targets.values() {
                        install_target(&dirs, target, &opts);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to parse cargo manifest {}", err);
            std::process::exit(1)
        }
    }
}

pub fn install_target(dirs: &InstallDirs, target: &Target, opts: &Options) {
    let install_dir = target
        .install_dir
        .as_deref()
        .map(|path| convert_to_path(path, dirs, Path::new("")));

    match target.type_ {
        Some(TargetType::Run) => match &target.target_file {
            Some(file) => {
                eprintln!(
                    "-- Executing steps for {}",
                    file.as_os_str().to_str().unwrap_or("<non unicode>")
                );
                if !opts.dry_run {
                    let mut cmd = Command::new(file);
                    cmd.envs(dirs.as_env());
                    if opts.verbose {
                        cmd.env("_VERBOSE", "1");
                    }
                    if let Some(dir) = &install_dir {
                        cmd.current_dir(dir);
                    }

                    match cmd.status() {
                        Ok(term) => match term.code() {
                            Some(0) | Some(20) => {}
                            Some(2) => {
                                eprintln!("  -- Failed (target returned exit code 2)");
                            }
                            Some(10) => {
                                eprintln!("  -- Skipped")
                            }
                            Some(c) => {
                                eprintln!("  -- Failed (target returned exit code {})", c);
                                std::process::exit(1)
                            }
                            None => {
                                eprintln!("  -- Failed (Unexpected termination)");
                                std::process::exit(1)
                            }
                        },
                        Err(e) => {
                            eprintln!("  -- Failed {}", e);
                            std::process::exit(1)
                        }
                    }
                }
            }
            None => {
                eprintln!("Failed to parse target, run targets require a file");
                std::process::exit(1)
            }
        },
        Some(s) => {
            let dir = s.get_install_root(dirs, opts).unwrap();
            let target_file = convert_to_path(
                target.installed_path.as_deref().unwrap(),
                dirs,
                install_dir.as_deref().unwrap_or(dir),
            );
            if target.privileged {
                match opts.install_privileged {
                    Some(false) => return,
                    None if opts.user_prefix => return,
                    _ => (),
                }
            }

            if target.directory {
                if let Some(src) = &target.target_file {
                    eprintln!(
                        "-- Installing directory {} to {}",
                        src.as_os_str().to_str().unwrap_or("<non unicode>"),
                        target_file.as_os_str().to_str().unwrap_or("<non unicode>")
                    )
                } else {
                    eprintln!(
                        "-- Creating directory {}",
                        target_file.as_os_str().to_str().unwrap_or("<non unicode>")
                    )
                }
            } else if let Some(src) = &target.target_file {
                eprintln!(
                    "-- Installing {} to {}",
                    src.as_os_str().to_str().unwrap_or("<non unicode>"),
                    target_file.as_os_str().to_str().unwrap_or("<non unicode>")
                )
            } else {
                eprintln!("Invalid target, no source file given, but one is expected");
                return;
            }
            if !opts.dry_run {
                let mut mode = None;
                if let Some(s) = &target.mode {
                    mode = Some(s.clone());
                }
                if let Some(m) = &opts.mode {
                    if let Some(mode) = &mut mode {
                        *mode += ",";
                        *mode += m;
                    } else {
                        mode = Some(m.clone());
                    }
                }
                if let Some(s) = &opts.install {
                    let mut cmd = Command::new(s);
                    if let Some(s) = &opts.strip {
                        if let Some(true) = target.strip {
                            let mut strip_arg = OsString::from("--strip-program=");
                            strip_arg.push(s);
                            cmd.arg("-s");
                            cmd.arg(strip_arg);
                        }
                    }

                    if !opts.no_create_dirs {
                        cmd.arg("-D");
                    }

                    if opts.verbose {
                        cmd.arg("-v");
                    }

                    if let Some(mode) = &mode {
                        cmd.arg("-m");
                        cmd.arg(mode);
                    }

                    if target.directory {
                        if let Some(s) = &target.target_file {
                            cmd.arg(s);
                        } else {
                            cmd.arg("-d");
                        }
                    } else if let Some(src) = &target.target_file {
                        cmd.arg("-T");
                        cmd.arg(src);
                    } else {
                        panic!();
                    }
                    cmd.arg(&target_file);
                    match cmd.status() {
                        Ok(c) => {
                            match c.code() {
                                Some(0) => (),
                                Some(x) => {
                                    eprintln!(
                                        "  -- Failed, install program exited with code {}",
                                        x
                                    );
                                }
                                None => {
                                    #[cfg(unix)]
                                    {
                                        if let Some(x) = c.signal() {
                                            // SAFETY:
                                            // libc::strsignal, which calls strsignal from the C Standard library, cannot cause undefined behaviour
                                            // Additionally, it is guaranteed, by the C Standard, to return pointer to a null terminated string
                                            eprintln!(
                                                "   -- Failed, install program recieved signal {}",
                                                unsafe { CStr::from_ptr(libc::strsignal(x)) }
                                                    .to_string_lossy()
                                            );
                                            return;
                                        }
                                    }

                                    eprintln!("   -- Failed, unknown result");
                                    return;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("    -- Failed, {}", e)
                        }
                    }

                    // if let Some(mode) = &mode {
                    //     set_permissions(
                    //         &target_file,
                    //         mode,
                    //         target.type_ == Some(TargetType::Bin)
                    //             || target.type_ == Some(TargetType::SBin)
                    //             || target.directory,
                    //     )
                    // }
                } else {
                    match do_internal_install(
                        target.target_file.as_deref(),
                        target_file,
                        opts,
                        target,
                        &mode,
                    ) {
                        Ok(()) => return,
                        Err(e) => {
                            eprintln!("Failed to install target {}", e);
                            return;
                        }
                    }
                }

                for alias in target.installed_aliases.iter().flatten() {
                    if let Err(_) = create_alias(alias, &target_file, opts, target.directory) {
                        eprintln!(
                            "   -- Failed to create alias {}",
                            alias.as_os_str().to_str().unwrap_or("<non unicode>")
                        );
                        return;
                    }
                }
            }
        }
        None => {}
    }
}

pub fn convert_to_path(input: &Path, dirs: &InstallDirs, primary: &Path) -> PathBuf {
    if input.has_root() {
        input.to_owned()
    } else {
        let mut components = input.components();

        match components.next() {
            Some(Component::Normal(s))
                if s == OsStr::new("<prefix>")
                    || s == OsStr::new("@prefix@")
                    || s == OsStr::new("${prefix}") =>
            {
                dirs.prefix
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<exec_prefix>")
                    || s == OsStr::new("@exec_prefix@")
                    || s == OsStr::new("${exec_prefix}") =>
            {
                dirs.exec_prefix
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<bindir>")
                    || s == OsStr::new("@bindir@")
                    || s == OsStr::new("${bindir}") =>
            {
                dirs.bindir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<sbindir>")
                    || s == OsStr::new("@sbindir@")
                    || s == OsStr::new("${sbindir}") =>
            {
                dirs.sbindir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<libdir>")
                    || s == OsStr::new("@libdir@")
                    || s == OsStr::new("${libdir}") =>
            {
                dirs.libdir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<libexecdir>")
                    || s == OsStr::new("@libexecdir@")
                    || s == OsStr::new("${libexecdir}") =>
            {
                dirs.libexecdir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<includedir>")
                    || s == OsStr::new("@includedir@")
                    || s == OsStr::new("${includedir}") =>
            {
                dirs.includedir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<datarootdir>")
                    || s == OsStr::new("@datarootdir@")
                    || s == OsStr::new("${datarootdir}") =>
            {
                dirs.datarootdir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<datadir>")
                    || s == OsStr::new("@datadir@")
                    || s == OsStr::new("${datadir}") =>
            {
                dirs.datadir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<mandir>")
                    || s == OsStr::new("@mandir@")
                    || s == OsStr::new("${mandir}") =>
            {
                dirs.mandir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<infodir>")
                    || s == OsStr::new("@infodir@")
                    || s == OsStr::new("${infodir}") =>
            {
                dirs.infodir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<docdir>")
                    || s == OsStr::new("@docdir@")
                    || s == OsStr::new("${datarootdir}") =>
            {
                dirs.docdir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<localedir>")
                    || s == OsStr::new("@localedir@")
                    || s == OsStr::new("${localedir}") =>
            {
                dirs.localedir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<localstatedir>")
                    || s == OsStr::new("@localstatedir@")
                    || s == OsStr::new("${localstatedir}") =>
            {
                dirs.localstatedir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<sharedstatedir>")
                    || s == OsStr::new("@sharedstatedir@")
                    || s == OsStr::new("${sharedstatedir}") =>
            {
                dirs.sharedstatedir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(Component::Normal(s))
                if s == OsStr::new("<sysconfdir>")
                    || s == OsStr::new("@sysconfdir@")
                    || s == OsStr::new("${sysconfdir}") =>
            {
                dirs.sysconfdir
                    .components()
                    .chain(components)
                    .collect::<PathBuf>()
            }
            Some(c) => primary
                .components()
                .chain(std::iter::once(c))
                .chain(components)
                .collect::<PathBuf>(),
            None => primary.to_owned(),
        }
    }
}

pub fn set_permissions<P1: AsRef<Path>>(
    #[allow(unused_variables)] target: P1,
    #[allow(unused_variables)] mode: &str,
    #[allow(unused_variables)] exec: bool,
) {
    #[cfg(unix)]
    {
        let dest_permissions = std::fs::metadata(target.as_ref()).unwrap().permissions();
        let umask = unsafe { libc::umask(0) };
        let mode = if mode.starts_with(|c: char| c.is_digit(8)) {
            u32::from_str_radix(&*mode, 8).unwrap() & !umask
        } else if mode.starts_with('=') && mode[1..].starts_with(|c: char| c.is_digit(8)) {
            u32::from_str_radix(&mode[1..], 8).unwrap()
        } else if mode.starts_with('+') && mode[1..].starts_with(|c: char| c.is_digit(8)) {
            u32::from_str_radix(&mode[1..], 8).unwrap() | dest_permissions.mode()
        } else if mode.starts_with('-') && mode[1..].starts_with(|c: char| c.is_digit(8)) {
            dest_permissions.mode() & !u32::from_str_radix(&mode[1..], 8).unwrap()
        } else {
            let mut mode_bits = dest_permissions.mode();
            for s in mode.split(",") {
                let mut chars = s.chars();
                let mut type_mask = 0;
                let mut cmode = 0;
                let mut modifier = ' '; // Not valid
                while let Some(c) = chars.next() {
                    if c == '=' || c == '+' || c == '-' {
                        modifier = c;
                        break;
                    }
                    match c {
                        'u' => type_mask |= 0o4700,
                        'g' => type_mask |= 0o2070,
                        'o' => type_mask |= 0o1007,
                        'a' => type_mask |= 0o7777,
                        _ => {
                            eprintln!("Invalid mode {}", mode);
                            std::process::exit(1)
                        }
                    }
                    if type_mask == 0 {
                        type_mask = 0o7777 & !umask;
                    }
                }
                for c in chars {
                    match c {
                        'r' => cmode |= 0o444,
                        'w' => cmode |= 0o222,
                        'x' => cmode |= 0o111,
                        'X' => {
                            cmode |= if mode_bits & 0o111 != 0 || exec {
                                0o111
                            } else {
                                0
                            }
                        }
                        's' => cmode |= 0o6000,
                        't' => cmode |= 0o1000,
                        _ => {
                            eprintln!("Invalid mode {}", mode);
                            std::process::exit(1)
                        }
                    }
                }

                match modifier {
                    '=' => mode_bits = (cmode & type_mask) | (mode_bits & 0o2000),
                    '+' => mode_bits |= cmode & type_mask,
                    '-' => mode_bits &= !(cmode & type_mask),
                    _ => {
                        eprintln!("Invalid mode {}", mode);
                        std::process::exit(1)
                    }
                }
            }
            mode_bits
        };
        std::fs::set_permissions(target, Permissions::from_mode(mode)).unwrap();
    }
}

pub fn create_alias<P1: AsRef<Path>, P2: AsRef<Path>>(
    src: P1,
    dest: P2,
    opts: &Options,
    #[allow(unused_variables)] dir: bool,
) -> std::io::Result<()> {
    if !opts.dry_run {
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(src, dest)
        }
        #[cfg(windows)]
        {
            if !_dir {
                std::os::windows::fs::symlink_file(src, dest)
            } else {
                std::os::windows::fs::symlink_dir(src, dest)
            }
        }
        #[cfg(not(any(unix, windows)))]
        {
            panic!("Unsupported operating system")
        }
    } else {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
struct InstallError;

impl Display for InstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Installation Error")
    }
}

impl Error for InstallError {}

pub fn do_internal_install<P1: AsRef<Path>, P2: AsRef<Path>>(
    src: Option<P1>,
    dest: P2,
    opts: &Options,
    target: &Target,
    mode: &Option<String>,
) -> std::io::Result<()> {
    if !opts.dry_run {
        if !opts.force {
            let src_md = src.as_ref().map(metadata).transpose();
            let dest_md = metadata(dest.as_ref());
            match (
                src_md.and_then(|m| m.map(|m| m.modified()).transpose()),
                dest_md.and_then(|m| m.modified()),
            ) {
                (Ok(Some(src_time)), Ok(dest_time)) => {
                    if src_time < dest_time {
                        return Ok(());
                    }
                }
                (_, _) => (),
            }
        }
        if target.directory {
            if !opts.no_create_dirs {
                fs::create_dir_all(dest.as_ref())?;
            }
            if let Some(src) = src {
                for p in fs::read_dir(src.as_ref())? {
                    let buf = p?.path();
                    let name = buf.file_name().unwrap();
                    let mut dest_item = dest.as_ref().to_path_buf();
                    dest_item.push(name);
                    do_internal_install(Some(buf), dest_item, opts, target, mode)?;
                }
            }
        } else if let Some(src) = src {
            fs::copy(src, dest.as_ref())?;
        } else {
            return Err(std::io::Error::new(ErrorKind::NotFound, InstallError));
        }

        if let Some(mode) = mode {
            set_permissions(
                &dest,
                mode,
                target.type_ == Some(TargetType::Bin)
                    || target.type_ == Some(TargetType::SBin)
                    || target.directory,
            );
        }

        if !target.directory {
            if let Some(s) = &opts.strip {
                let mut cmd = Command::new(s);
                cmd.arg("-s");
                cmd.arg(dest.as_ref());
                cmd.stdin(Stdio::null());
                cmd.stdout(Stdio::null());
                cmd.stderr(Stdio::null());
                cmd.status()?;
            }
        }

        Ok(())
    } else {
        Ok(())
    }
}
