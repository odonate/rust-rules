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

Then add the toolchain and plugin config to `.plzconfig`:
```
[BuildConfig]
rust-toolchain = //third_party/rust:toolchain

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
	root = "src/main.rs",
	deps = [
	    ":lib",
		"//third_party/rust:<rust_crate_name>",
	],
)
```

## Configuration
Configurability is a WIP.

## General notes
Rust Rules is based heavily on Cargo in its invocation of the `rustc` compiler. Care should be taken when defining third-party crates to ensure the correct depencies and their versions are supplied, required conditional compilation features are set, and correct Rust editions are used. This information can be found in the `Cargo.toml` file of the third-party crate. Unfortunately, at present, Rust Rules does not parse the `Cargo.toml` file to infer this information, so it must be supplied manually.

## Contributing
Contributions are welcome! Please open or submit a pull request with your changes. Ensure that your code follows the existing style and includes tests where applicable.
