# cargo-native-install

Cargo plugin to perform native installation of binary and library packages. 


## Format

All information used by `cargo-native-install` is stored in the project's `Cargo.toml`. 

Each "target" for installation is specified by a key under `[package.metadata.install-targets]`. 

Each key is the name of a target.

Additionally, each binary and the library target (if any) is automatically an installation target for this program, unless the exclude field of the `install-targets` subobject is corresponding to the target is set to true. This is only the case for library targets if the `crate-type` field contains `cdylib` or `staticlib`. For these library targets, if both are present, the staticlib installation target is named `<library-name>-static` and the cdylib installation target is `<library-name>-dynamic`. Installation candidates are not generated, but may be manually created for `rlib`, `dylib`, and `proc-macro` libraries, by appending `-<type>` to the library name.  
