# cargo-native-install

Cargo plugin to perform native installation of binary and library packages. 

## Version Policy

Changes to the described interface below are versioned according to the following policy

Semver Patch (except for `0.x`):
* Bug fixes, security updates

Semver Minor (including patches for `0.x`):
* Changes to any behaviours recognized as reserved
   - This exhaustively includes: The behaviour of unmentioned return codes from `run` targets, the list of directory environment variables passed to `run` targets and cargo, changes to the directories which are used in subsitutions of `installed_path`, whether directory substitutions are applied to `install_dir`.
* Any changes to the exposed Command Line Interface. 
* Addition of a new key with a default to entries in the `install-targets` array.


Semver Major (including `0.x`):
* A change in any specified behaviour defined under the "Format" section, other than those specified as reserved.

## Command Line Interface

(Note: An up-to-date comprehensive document can be found in the manual page, or by `cargo-native-install --help`)

Usage: cargo-native-install [options]...
* Installs the current cargo project into native system directories (like GNU make install or cmake --install)

Options:
* --help: Prints this message, and exits
* --version: Prints version information, and exits
* --dry-run: Show the results of each install operation, but do not perform any operations
* --user-prefix: Default prefix to ~/.local, instead of a system-wide dir
* --prefix=<prefix>: Sets the prefix for installation operations
* --bindir=*dir*: Use dir as the directory to install binary programs. Either an absolute path, or a path relative to prefix. (defaults to bin)
* --libdir=*dir*: Use dir as the directory to install libraries. Either an *absolute path, or a path relative to prefix (defaults to lib)
* --sbindir=*dir*: Use dir as the directory to install system administrator programs. Either an absolute path, or a path relative to prefix (defaults to sbin)
* --libexecdir=*dir*: Use dir as the directory to install programs that aren't for direct use from the shell. Either an absolute path, or a path relative to prefix (defaults to libexec)
* --includedir=*dir*: Use dir as the directory to install header files. Either an absolute path, or a path relative to prefix (defaults to include)
* --datarootdir=*dir*: Use dir as the prefix for platform independent data, documentation, and manuals. Either an absolute path, or a path relative to prefix (defaults to share)
* --datadir=*dir*: Use dir as the directory to install platform independent data. Either an absolute path, or a path relative to the data root (defaults to the same directory as the data root)
* --mandir=*dir*: Use dir as the directory for installing manual pages. Either an absolute path, or a path relative to data root (defaults to man)
* --infodir=*dir*: Use dir as the directory for installing info pages. Either an absolute path, or a path relative to data root (defaults to info)
* --docdir=*dir*: Use dir as the directory for installing project documentation. Either an absolute path, or a path relative to data root (defaults to doc)
* --localedir=*dir*: Use dir as the directory for installing locale specific *information. Either an absolute path, or a path relative to data root (defaults to locale)
* --sysconfdir=*dir*: Use dir as the directory for system configuration files. Either an absolute path, or a path relative to the prefix (defaults to etc)
* --localstatedir=*dir*: Use dir as the directory for local system state. Either an absolute path, or a path relative to the prefix (defaults to var)
* --sharedstatedir=*dir*: Use dir as the directory for shared system state. Either an absolute path, or a path relative to the prefix (defaults to com)
* --manifiest-dir=*dir*: Indicates the directory to the cargo manifest.
* --no-strip: Do not strip programs, even if strip is found
* --without-strip: Same as --no-strip
* --strip=<prg>: Use <prg> to strip, instead of the default (strip)
* --install=<prg>: Use <prg> to install programs, instead of the default (install)
* --internal-install: Do not invoke any programs to install. Instead, copy files natively. This is the default if install is not found, and `--install` is not provided
* --mode=<mode>: Force installed files to use <mode> in the form of a chmod *mode (X is the executable bit if the file is a binary target, or a directory)
* --no-create: Do not create installed directories. Also do not create any prefix directories
* --verbose: Print messages for each action
* --force: Install all files, even if this would replace files that are newer
* --no-privileged: Do not install privileged binaries (those installed to sbin)
* --privileged: Install privilged binaries to sbindir, even if a user-specific prefix is used
* --target=*target*: Install only this target
* --no-libexec: Install libexec targets to bin instead
* --no-sbin: Install sbin targets to bin instead of sbin (note that this does not enable privileged binaries)
* --arch-prefix[=target]: Install bin, lib, include, libexec, sbin targets to an an architecture specific prefix.
* --build: Build the package before installing. An environment variable corresponding to each directory is set during the build. Use of `--build` and `--build-only` is deprecated in favor of `--config` and `config.toml`, and using build scripts for configuring build-time installation directories. 
* --build-only: Build the package without installing. Like --build, environment variables will be set with all the directories. Use of `--build` and `--build-only` is deprecated in favor of `--config` and `config.toml`, and using build scripts for configuring build-time installation directories. 
* --shared=lib: Treat cdylib targets as library targets by default and install to libdir. This is the default on unix-like targets
* --shared=bin: Treat cdylib targets as binary targets by default and install to bindir. This is the default on windows
* --out-dir=*dir*: Consider cargo targets to be stored in *dir* instead of *manifest-dir*/target
* --release: Consider cargo targets to have been built in release mode *(default)
* --debug: Consider cargo targets to have been built in debug mode
* --config=*file*: If *file* exists, then read default installation directories from that path, rather than `config.toml`.

Environment:
* prefix: Install directories may be specified as environment variables, as well as with options. If both the environment variable and the CLI option is present, the option takes precedence
* exec_prefix: Similar to prefix.
* bindir: Similar to prefix.
* libdir: Similar to prefix.
* sbindir: Similar to prefix.
* libexecdir: Similar to prefix.
* includedir: Similar to prefix.
* datarootdir: Similar to prefix.
* datadir: Similar to prefix.
* docdir: Similar to prefix.
* mandir: Similar to prefix.
* infodir: Similar to prefix.
* localedir: Similar to prefix.
* localstatedir: Similar to prefix.
* sharedstatedir: Similar to prefix.
* sysconfdir: Similar to prefix.
* runstatedir: If specified, the variable is propagate to run targets, and to cargo. Has no further effect on the program
* PATH: Searches for install and strip in these paths

### config.toml

By default, `cargo-native-install` will look for a file name `config.toml` in the working directory. The name and path to the file can be configured by specifying the `--config` option, but the behaviour remains the same.
If a file exists, it will be parsed (as toml), and install directories will be read from the `[dir]` object. Where the directories are present, these will override the default setting for that directory. Note that this will not affect directories specified by environment variables, or command line flags.

This mechanism is intended to replace the `--build` and `--build-only` flags, which could be used to build the project with the directories specified in the environment. Using config.toml, the package would need to provide a build script that parses the file and sets the appropriate environment variables. 
Because the file specifies the default directories, and can be overriden by individual options and environment variables, this can be used to set a "sysroot", where the project is built using a standard prefix, and then installed into a different path, which can then be used with chroot, or to a mount point on a different partition. 


## Format

All information used by `cargo-native-install` is stored in the project's `Cargo.toml`. 

Each "target" for installation is specified by a key under `[package.metadata.install-targets]`. 

Each key is the name of a target.

Additionally, each binary and the library target (if any) is automatically an installation target for this program, unless the exclude field of the `install-targets` subobject is corresponding to the target is set to true. This is only the case for library targets if the `crate-type` field contains `cdylib` or `staticlib`. For these library targets, if both are present, the staticlib installation target is named `<library-name>-staticlib` and the cdylib installation target is `<library-name>-cdylib`. Installation candidates are not generated, but may be manually created for `rlib`, `dylib`, and `proc-macro` libraries, by appending `-<type>` to the library name.  

If a key is specified, then it may have one of several fields, the defaults for each type of binary/library target are provided:
-  `type`: May be one of "bin", "sbin", "library", "libexec", "shared", "include", "data", "doc", "man", "info", "sysconfig", or "run".
    - Defaults to "bin" for binary targets without privileged set, and "sbin" for targets with privileged set
    - Defaults to "library" for staticlib targets, as well as `rlib` targets, and "shared" for cdylib targets and `dylib` targets.
- `privileged`: Do not install when `--user-prefix` or `--no-privileged` is specified (unless `--privileged` is specified). Defaults to `false` always.
- `directory`: Create the target as a directory, and copy source files to it. Defaults to `false` if not specified.
- `install_dir`: The path to be installed into. Defaults to the installation directory for the type, unless the type is `run` (note that no substitutions are performed on `install_dir`)
    - `bin` targets use `bindir` which defaults to `<exec_prefix>/bin`
    - `library` targets use `libdir` which defaults to `<exec_prefix>/lib`
    - `sbin` targets use `sbindir` which defaults to `<exec_prefix>/sbin`
    - `libexec` targets use `libexecdir` which defaults to `<exec_prefix>/lib` 
    - `data` targets use `datadir` which defaults to `<datarootdir>`
    - `doc` targets use `docdir` which defaults to `<datarootdir>/doc/<package-name>`
    - `man` targets use `mandir` which defaults to `<datarootdir>/man`
    - `info` targets use `infodir` which defaults to `<datarootdir>/info`
    - `sysconfig` targets use `sysconfdir` which defaults to `<prefix>/etc` (which special cases for `/opt/...` and `/usr/...` prefixes). 
    - Specifying a directory that starts with the name of an install diretory enclosed in `<>`, `${}`, or `@@`, or any ASCII identifier that ends in `dir` (case insensitive) is reserved for future releases.
- `mode`: Sets the mode to install as, in a form acceptable to `chmod(1)` (note: only guaranteed to be effective on unix platforms). 
    - For binary targets, this defaults to "=rwx".
    - For all library targets, this defaults to "=rw". 
- `installed_path`: The path to the installed file. If it starts with the name of a install directory (like prefix, exec_prefix, or bindir), enclosed in either `<>`, `@@` or `${}` (as `<prefix>`, `@exec_prefix@`, or `${bindir}`), it will be replaced with that directory. Otherwise, if it's a relative path, it is resolved by the `install_dir`. By default, this is the name of the target file. 
    - A path which starts with any ASCII identifier that ends in `dir` enclosed within `<>`, `@@`, or `${}` which is not substituted as above is reserved. 
- `target_file`: The file in the source directory, relative to `Cargo.toml`. By default, this is the file built for this target by cargo. Must exist for non-generated targets that do not have `directory` set.
- `installed_aliases`: After installing the target, create a symbolic link to it with each of the given names. 
- `exclude`: If set, disable this target. If set, all other options are ignored.

For `run` targets, the `target_file` must be an executable program. It is executed with no parameters, and in the `install_dir` if explicitly specified (otherwise in the current directory). 
An environment variable is set for each of the installation directories to the specified one, as absolute paths. 
- Any environment variable that is an ascii identifier that ends in `dir` is reserved for future use. Such variables may be expected by `run` targets to be an absolute path name if defined, but such targets may make no further assumptions about the existance or content of such environment variables
Additionally, the verbose flag is passed into the program by setting the `_VERBOSE` environment variable to `1`. Note that no requirement is specified that this environment variable be respected, or even meaningful to the program. 

All environment variables set for `run` targets are also set when invoking `cargo` for `cargo-native-install --build` and `--build-only`. 

The meaning of exit codes from a run target are as follows:
- `0`: successful execution, no report, continue installing
- `1`: Error execution, report error, installation fails. 
- `2`: Error execution, report error, continue installing (not fatal)
- `10`: target skipped, report, continue installing
- `20`: target skipped, no report, continue installing
- Any other code: Reserved for future versions. Error execution, report error, installation fails
    - Such codes may be assigned further meaning in future versions. `run` targets should not return such a code. 
- If a run target is terminated by a signal, an error is reported, and installation fails. 

