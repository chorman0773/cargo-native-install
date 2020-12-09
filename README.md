# cargo-native-install

Cargo plugin to perform native installation of binary and library packages. 


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
- `mode`: Sets the mode to install as, in a form acceptable to `chmod(1)` (note: only guaranteed to be effective on unix platforms). 
    - For binary targets, this defaults to "=rwx".
    - For all library targets, this defaults to "=rw". 
- `installed_path`: The path to the installed file. If it starts with the name of a install directory (like prefix, exec_prefix, or bindir), enclosed in either `<>`, `@@` or `${}` (as `<prefix>`, `@exec_prefix@`, or `${bindir}`), it will be replaced with that directory. Otherwise, if it's a relative path, it is resolved by the `install_dir`. By default, this is the name of the target file. 
- `target_file`: The file in the source directory, relative to `Cargo.toml`. By default, this is the file built for this target by cargo. Must exist for non-generated targets that do not have `directory` set.
- `installed_aliases`: After installing the target, create a symbolic link to it with each of the given names.
- `exclude`: If set, disable this target. If set, all other options are ignored.

For `run` targets, the `target_file` must be an executable program. It is executed with no parameters, and in the `install_dir` if explicitly specified (otherwise in the current directory). 
An environment variable is set for each of the installation directories to the specified one, as absolute paths. 
Additionally, the verbose flag is passed into the program by setting the `verbose` environment variable to `1`. Note that no requirement is specified that this environment variable be respected, or even meaningful to the program. 
The meaning of exit codes from a run target are as follows:
- `0`: successful execution, no report, continue installing
- `2`: error execution, report error, continue installing (not fatal)
- `10`: target skipped, report, continue installing
- `20`: target skipped, no report, continue installing
- Any other code: error execution, report error, installation fails.
- If a run target is terminated by a signal, an error is reported, and installation fails. 

