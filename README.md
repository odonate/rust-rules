# Rust Rules
This repo provides Rust build rules for the [Please](https://please.build) build system.

## Basic usage
First add the plugin to your project. In `plugins/BUILD`:
```python
plugin_repo(
    name = "rust",
    owner = "odonate",
    revision = "<Some git tag, commit, or other reference>",
)
```

Set up the Rust toolchain for your project in `third_party/rust`:
```python
subinclude("///rust//build_defs:rust")

rust_toolchain(
    name = "toolchain",
    hashes = ["<hash>"],
    version = "X.XX.X",
    visibility = ["PUBLIC"],
)
```

Then add the plugin config to `.plzconfig`:
```ini
[Plugin "rust"]
Target = //plugins:rust
```

You can then compile and test Rust libraries like so:
```python
subinclude("///rust//build_defs:rust")

rust_library(
    name = "lib",
    root = "src/lib.rs",
    modules = [
        "src/module_a.rs",
        "src/module_a/sub_module_a.rs",
        "src/module_b.rs",
        "src/module_b/sub_module_b.rs",
    ],
)

rust_test(
    name = "lib",
    root = "src/lib.rs",
    modules = [
        "src/module_a.rs",
        "src/module_a/sub_module_a.rs",
        "src/module_b.rs",
        "src/module_b/sub_module_b.rs",
    ],
)
```

You can define third-party crates using `rust_crate`:
```python
subinclude("///rust//build_defs:rust")

rust_crate(
    name="libc",
    crate="libc",
    version="0.2.155",
    build_script="build.rs",
    edition="2015",
)

rust_crate(
    name="cfg_if",
    crate="cfg-if",
    version="1.0.0",
    edition="2018",
)

rust_crate(
    name="getrandom",
    crate="getrandom",
    version="0.2.15",
    edition="2018",
    features=["std"],
    deps=[":cfg_if", ":libc"],
)

rust_crate(
    name="rand_core",
    crate="rand_core",
    version="0.6.4",
    edition="2018",
    features=["alloc", "getrandom", "std"],
    deps = [":getrandom"],
)
```

To compile a binary, you can use `rust_binary`:
```python
subinclude("///rust//build_defs:rust")

rust_binary(
    name = "bin",
    main = "src/main.rs",
    deps = [
        ":lib",
        "//third_party/rust:<rust_crate_name>",
    ],
)
```

## Configuration
Plugins are configured through the Plugin section like so:
```ini
[Plugin "rust"]
SomeConfig = some-value
```
The available configuration options are:

### Rustc
The path to the `rustc` compiler to use. Defaults to the toolchain's rustc.
```ini
[Plugin "rust"]
Rustc = //third_party/rust:toolchain_rustc
```

### StdLib
The path to the `stdlib` to be linked by the compiler. Defaults to the toolchain's stdlib.
```ini
[Plugin "rust"]
StdLib = //third_party/rust:toolchain_stdlib
```

## General notes
Rust Rules is based heavily on Cargo in its invocation of the `rustc` compiler. Care should be taken when defining third-party crates to ensure the correct depencies and their versions are supplied, required conditional compilation features are set, and correct Rust editions are used. This information can be found in the `Cargo.toml` file of the third-party crate. Unfortunately, at present, Rust Rules does not parse the `Cargo.toml` file to infer this information, so it must be supplied manually.

## Contributing
Contributions are welcome! Please open or submit a pull request with your changes. Ensure that your code follows the existing style and includes tests where applicable.

### Extra Features for Contribution
Here are some extra features that would be valuable additions to this project:

- **Crate Types**: The `rust_crate` rule currently supports the following crate types `lib`, `rlib`, `proc-macro`. Adding support for other crate types, would be useful. Other types to support: `bin`, `staticlib`, `dylib`, `cdylib`.

- **Improved Linking**: Currently, there is a hacky method for getting the dependency directory for linking crates with the -L flag. A cleaner solution would enhance the build process.

- **Codegen Flags**: Implementing support for the codegen `-C metadata` and `-C extra-filename` flags would better align with how Cargo fingerprints external crates and dependencies.

- **Build Script Outputs**: Currently, only `cargo:rustc-cfg:` is supported in build scripts. Adding support for other outputs like `cargo:rustc-rerun-if-xxx`, `cargo:rustc-link-xxx`, and `cargo:rustc-env` would be beneficial. Additionally, parsing the `Cargo.toml` to check if any values should be overridden would provide more comprehensive functionality.

- **Target (OS and Architecture) Compatibility**: This project has primarily been tested on unknown-linux-gnu x86_64 architecture. It would be nice to test and support other targets to ensure cross-platform compatibility.
