subinclude("//build_defs:rust")

rust_toolchain(
    name = "toolchain",
    hashes = ["03d89cfdd013996f9911165ffb5790c5fd16f059"],
    version = "1.78.0",
    visibility = ["PUBLIC"],
)

# rust_toolchain(
#     name = "toolchain_nightly",
#     # The hash changes all the time.
#     hashes = ["af79758278cfe385ab0d4520f2c64f4ae7f9c7c6"],
#     version = "nightly",
#     visibility = ["PUBLIC"],
# )

#### Used by examples/rustc_guessing_game ####
rust_crate(
    name="libc",
    crate="libc",
    version="0.2.155",
    edition="2015",
    features = [
        "default",
        "std",
    ],
    build_root="build.rs",
)

rust_crate(
    name="cfg_if",
    crate="cfg-if",
    version="1.0.0",
    edition="2018",
)

rust_crate(
    name="ppv_lite86",
    crate="ppv-lite86",
    version="0.2.17",
    edition="2018",
    features=["simd", "std"]
)

rust_crate(
    name="getrandom",
    crate="getrandom",
    version="0.2.15",
    edition="2018",
    deps=[
        ":cfg_if",
        ":libc",
    ],
    features=["std"],
)

rust_crate(
    name="rand_core",
    crate="rand_core",
    version="0.6.4",
    edition="2018",
    features=["alloc", "getrandom", "std"],
    deps = [":getrandom"],
)

rust_crate(
    name="rand_chacha",
    crate="rand_chacha",
    version="0.3.1",
    edition="2018",
    deps=[
        ":ppv_lite86",
        ":rand_core",
    ],
    features=["std"],
)

rust_crate(
    name="rand",
    crate="rand",
    version="0.8.5",
    edition="2018",
    features=["alloc", "default", "getrandom", "libc", "rand_chacha", "small_rng", "std", "std_rng"],
    deps=[
        ":libc",
        ":rand_chacha",
        ":rand_core",
    ],
)

#### Used by examples/rustc_calculator ####
rust_crate(
    name = "unicode_ident",
    crate = "unicode-ident",
    edition = "2018",
    version = "1.0.12",
)

rust_crate(
    name = "proc_macro2",
    build_root = "build.rs",
    crate = "proc-macro2",
    edition = "2021",
    features = ["default", "proc-macro"],
    version = "1.0.85",
    deps = [":unicode_ident"],
)

rust_crate(
    name = "autocfg",
    crate = "autocfg",
    edition = "2015",
    version = "1.3.0",
)

rust_crate(
    name = "pin_project_lite",
    crate = "pin-project-lite",
    edition = "2018",
    version = "0.2.14",
    warns = [
        "unreachable_pub",
        "clippy::undocumented_unsafe_blocks",
        "clippy::transmute_undefined_repr",
        "clippy::trailing_empty_array",
        "single_use_lifetimes",
        "rust_2018_idioms",
        "clippy::pedantic",
        "non_ascii_idents",
        "clippy::inline_asm_x86_att_syntax",
        "improper_ctypes_definitions",
        "improper_ctypes",
        "clippy::default_union_representation",
        "clippy::as_ptr_cast_mut",
        "clippy::all",
    ],
    allows = [
        "clippy::type_complexity",
        "clippy::too_many_lines",
        "clippy::too_many_arguments",
        "clippy::struct_field_names",
        "clippy::struct_excessive_bools",
        "clippy::single_match_else",
        "clippy::single_match",
        "clippy::similar_names",
        "clippy::module_name_repetitions",
        "clippy::missing_errors_doc",
        "clippy::manual_range_contains",
        "clippy::manual_assert",
        "clippy::float_cmp",
        "clippy::doc_markdown",
        "clippy::declare_interior_mutable_const",
        "clippy::borrow_as_ptr",
        "clippy::bool_assert_comparison",
    ],
)

rust_crate(
    name = "bytes",
    crate = "bytes",
    edition = "2018",
    version = "1.6.0",
    features = [
        "default",
        "std",
    ],
)

rust_crate(
    name = "futures_core",
    crate = "futures-core",
    edition = "2018",
    version = "0.3.30",
    features = [
        "alloc",
        "default",
        "std",
    ],
)

rust_crate(
    name = "once_cell",
    crate = "once_cell",
    edition = "2021",
    version = "1.19.0",
    features = [
        "alloc",
        "default",
        "race",
        "std",
    ],
)

rust_crate(
    name = "scopeguard",
    crate = "scopeguard",
    edition = "2015",
    version = "1.2.0",
)

rust_crate(
    name = "smallvec",
    crate = "smallvec",
    edition = "2018",
    version = "1.13.2",
)

rust_crate(
    name = "parking_lot_core",
    build_root = "build.rs",
    crate = "parking_lot_core",
    edition = "2021",
    version = "0.9.10",
    deps = [
        ":cfg_if",
        ":libc",
        ":smallvec",
    ],
)

rust_crate(
    name = "hashbrown",
    crate = "hashbrown",
    edition = "2021",
    version = "0.14.5",
    features = ["raw"],
)

rust_crate(
    name = "hashbrown.0.12.3",
    crate = "hashbrown",
    version = "0.12.3",
    edition = "2021",
    features = [
        "raw",
    ],
)

rust_crate(
    name = "equivalent",
    crate = "equivalent",
    edition = "2015",
    version = "1.0.1",
)

rust_crate(
    name = "indexmap",
    crate = "indexmap",
    edition = "2021",
    version = "2.2.6",
    features = [
        "default",
        "std",
    ],
    deps = [
        ":equivalent",
        ":hashbrown",
    ],
    allows = [
        "clippy::style",
    ],
)

# https://github.com/indexmap-rs/indexmap/issues/289
rust_crate(
    name = "indexmap.1.9.3",
    crate = "indexmap",
    version = "1.9.3",
    edition = "2021",
    features = [
        #"default",
        #"std",
    ],
    deps = {
        "hashbrown": ":hashbrown.0.12.3",
    },
    build_root = "build.rs",
    build_deps = [
        ":autocfg",
    ],
)

rust_crate(
    name = "indexmap.1.2.0",
    crate = "indexmap",
    version = "1.2.0",
    edition = "2015",
)


rust_crate(
    name = "anyhow",
    crate = "anyhow",
    build_root = "build.rs",
    edition = "2018",
    version = "1.0.86",
    features = [
        "default",
        "std",
    ],
)

rust_crate(
    name = "either",
    crate = "either",
    edition = "2018",
    version = "1.12.0",
)

rust_crate(
    name = "lock_api",
    crate = "lock_api",
    edition = "2021",
    version = "0.4.12",
    features = ["atomic_usize", "default"],
    deps = [":scopeguard"],
    build_root = "build.rs",
    build_deps = [":autocfg"],
)

rust_crate(
    name = "parking_lot",
    crate = "parking_lot",
    edition = "2021",
    version = "0.12.3",
    features = ["default"],
    deps = [
        ":lock_api",
        ":parking_lot_core",
    ],
)

rust_crate(
    name = "itertools",
    crate = "itertools",
    edition = "2018",
    version = "0.12.1",
    features = ["use_alloc"],
    deps = [":either"],
)

rust_crate(
    name = "slab",
    crate = "slab",
    edition = "2018",
    version = "0.4.9",
    features = [
        "default",
        "std",
    ],
    build_root = "build.rs",
    build_deps = [
        ":autocfg",
    ],
)

rust_crate(
    name = "pin_utils",
    crate = "pin-utils",
    edition = "2018",
    version = "0.1.0",
)

rust_crate(
    name = "fnv",
    crate = "fnv",
    edition = "2015",
    version = "1.0.7",
    features = [
        "default",
        "std",
    ],
    src_root = 'lib.rs',
)

rust_crate(
    name = "quote",
    crate = "quote",
    edition = "2018",
    version = "1.0.36",
    features = [
        "default",
        "proc-macro",
    ],
    deps = [
        ":proc_macro2",
    ],
)

rust_crate(
    name = "futures_task",
    crate = "futures-task",
    edition = "2018",
    version = "0.3.30",
    features = [
        "alloc",
    ],
)

rust_crate(
    name = "itoa",
    crate = "itoa",
    edition = "2018",
    version = "1.0.11",
)

rust_crate(
    name = "http",
    crate = "http",
    edition = "2018",
    version = "0.2.12",
    deps = [
        ":bytes",
        ":fnv",
        ":itoa",
    ],
)

rust_crate(
    name = "syn",
    crate = "syn",
    edition = "2021",
    version = "2.0.66",
    features = [
        "clone-impls",
        "default",
        "derive",
        "extra-traits",
        "full",
        "parsing",
        "printing",
        "proc-macro",
        "visit-mut",
    ],
    deps = [
        ":proc_macro2",
        ":quote",
        ":unicode_ident",
    ],
)

rust_crate(
    name = "futures_util",
    crate = "futures-util",
    edition = "2018",
    features = ["alloc"],
    version = "0.3.30",
    deps = [
        ":futures_core",
        ":futures_task",
        ":pin_project_lite",
        ":pin_utils",
    ],
)

rust_crate(
    name = "num_cpus",
    crate = "num_cpus",
    edition = "2015",
    version = "1.16.0",
    deps = [
        ":libc",
    ],
)

rust_crate(
    name = "signal_hook_registry",
    crate = "signal-hook-registry",
    edition = "2015",
    version = "1.4.2",
    deps = [
        ":libc",
    ],
)

rust_crate(
    name = "mio",
    crate = "mio",
    edition = "2018",
    version = "0.8.11",
    features = ["net", "os-ext", "os-poll"],
    deps = [
        ":libc",
    ],
)

rust_crate(
    name = "socket2",
    crate = "socket2",
    edition = "2021",
    version = "0.5.7",
    features = ["all"],
    deps = [
        ":libc",
    ],
)

rust_crate(
    name = "rustversion",
    crate = "rustversion",
    version = "1.0.17",
    edition = "2018",
    crate_type = 'proc-macro',
    build_root = "build/build.rs",
    compile_env = {
        # TODO: absolute path would be better.
        'OUT_DIR': '../../target/debug/build/rustversion_1_0_17/out',
    },
)

rust_crate(
    name = "tracing_core",
    crate = "tracing-core",
    version = "0.1.32",
    edition = "2018",
    features = [
        "once_cell",
        "std",
    ],
    deps = [
        ":once_cell",
    ],
)

rust_crate(
    name = "futures_sink",
    crate = "futures-sink",
    version = "0.3.30",
    edition = "2018",
    features = [
        "alloc",
        "default",
        "std",
    ],
)

rust_crate(
    name = "bitflags",
    crate = "bitflags",
    version = "2.5.0",
    edition = "2021",
    features = [
        "std",
    ],
)

rust_crate(
    name = "bitflags.1.3.2",
    crate = "bitflags",
    version = "1.3.2",
    edition = "2018",
    features = [
        "default",
    ],
)

rust_crate(
    name = "httparse",
    crate = "httparse",
    version = "1.9.3",
    edition = "2018",
    build_root = "build.rs",
    features = [
        "default",
        "std",
    ],
)

rust_crate(
    name = "linux_raw_sys",
    crate = "linux-raw-sys",
    version = "0.4.14",
    edition = "2021",
    features = [
        "elf",
        "errno",
        "general",
        "ioctl",
        "no_std",
    ],
)

rust_crate(
    name = "errno",
    crate = "errno",
    version = "0.3.9",
    edition = "2018",
    deps = [
        ":libc",
    ],
)

rust_crate(
    name = "rustix",
    crate = "rustix",
    version = "0.38.34",
    edition = "2021",
    features = [
        "alloc",
        "default",
        "fs",
        "libc-extra-traits",
        "std",
        "use-libc-auxv",
    ],
    deps = {
        "bitflags": ":bitflags",
        "linux_raw_sys": ":linux_raw_sys",
        "libc": ":libc",
        "libc_errno": ":errno", # renamed
    },
    build_root = "build.rs",
    build_run_env = {
        # Found from <cargo_out_dir>.rustc_info.json
        "CARGO_CFG_TARGET_ARCH": "x86_64",
        "CARGO_CFG_TARGET_ENV": "gnu",
        "CARGO_CFG_TARGET_OS": "linux",
        "CARGO_CFG_TARGET_POINTER_WIDTH": "64",
        "CARGO_CFG_TARGET_ENDIAN": "little",
        "RUSTC": CONFIG.RUST.RUSTC,
        "TARGET": "x86_64-unknown-linux-gnu",
    },
    build_deps = [
        ":libc",
    ],
)

rust_crate(
    name = "tower_service",
    crate = "tower-service",
    version = "0.3.2",
    edition = "2018",
)

rust_crate(
    name = "regex_syntax",
    crate = "regex-syntax",
    version = "0.8.4",
    edition = "2021",
    features = [
        "std",
        "unicode-bool",
    ],
)

rust_crate(
    name = "prettyplease",
    crate = "prettyplease",
    version = "0.2.20",
    edition = "2021",
    build_root = "build.rs",
    build_compile_env = {
        "CARGO_PKG_VERSION": "0.1.0",
    },
    deps = [
        ":proc_macro2",
        ":syn",
    ],
)

rust_crate(
    name = "http_body",
    crate = "http-body",
    version = "0.4.6",
    edition = "2018",
    deps = [
        ":bytes",
        ":http",
        ":pin_project_lite",
    ],
)

rust_crate(
    name = "try_lock",
    crate = "try-lock",
    version = "0.2.5",
    edition = "2015",
)

rust_crate(
    name = "fastrand",
    crate = "fastrand",
    version = "2.1.0",
    edition = "2018",
    features = [
        "alloc",
        "default",
        "std",
    ],
)

rust_crate(
    name = "fixedbitset",
    crate = "fixedbitset",
    version = "0.4.2",
    edition = "2015",
)

rust_crate(
    name = "want",
    crate = "want",
    version = "0.3.1",
    edition = "2018",
    deps = [
        ":try_lock",
    ],
)

rust_crate(
    name = "petgraph",
    crate = "petgraph",
    version = "0.6.5",
    edition = "2018",
    deps = [
        ":fixedbitset",
        ":indexmap",
    ],
)

rust_crate(
    name = "futures_channel",
    crate = "futures-channel",
    version = "0.3.30",
    edition = "2018",
    features = [
        "alloc",
        "default",
        "std",
    ],
    deps = [
        ":futures_core",
    ],
)

rust_crate(
    name = "heck",
    crate = "heck",
    version = "0.5.0",
    edition = "2021",
)

rust_crate(
    name = "httpdate",
    crate = "httpdate",
    version = "1.0.3",
    edition = "2021",
)

rust_crate(
    name = "log",
    crate = "log",
    version = "0.4.21",
    edition = "2021",
)

rust_crate(
    name = "multimap",
    crate = "multimap",
    version = "0.10.0",
    edition = "2015",
)

rust_crate(
    name = "tower_layer",
    crate = "tower-layer",
    version = "0.3.2",
    edition = "2018",
)

rust_crate(
    name = "regex_automata",
    crate = "regex-automata",
    version = "0.4.7",
    edition = "2021",
    features = [
        "alloc",
        "meta",
        "nfa-pikevm",
        "nfa-thompson",
        "std",
        "syntax",
        "unicode-bool",
    ],
    deps = [
        ":regex_syntax",
    ],
)

rust_crate(
    name = "mime",
    crate = "mime",
    version = "0.3.17",
    edition = "2015",
)

rust_crate(
    name = "tempfile",
    crate = "tempfile",
    version = "3.10.1",
    edition = "2021",
    deps = [
        ":cfg_if",
        ":fastrand",
        ":rustix",
    ],
)

rust_crate(
    name = "serde",
    crate = "serde",
    version = "1.0.203",
    edition = "2018",
    features = [
        "default",
        "std",
    ],
    build_root = "build.rs",
    build_run_env = {
        "TARGET": "x86_64-unknown-linux-gnu",
    },
)

rust_crate(
    name = "matchit",
    crate = "matchit",
    version = "0.7.3",
    edition = "2021",
    features = [
        "default",
    ],
)

rust_crate(
    name = "percent_encoding",
    crate = "percent-encoding",
    version = "2.3.1",
    edition = "2018",
    features = [
        "alloc",
        "default",
        "std",
    ],
)

rust_crate(
    name = "sync_wrapper",
    crate = "sync_wrapper",
    version = "0.1.2",
    edition = "2018",
)

rust_crate(
    name = "memchr",
    crate = "memchr",
    version = "2.7.4",
    edition = "2021",
    features = [
        "alloc",
        "default",
        "std",
    ],
)

rust_crate(
    name = "base64",
    crate = "base64",
    version = "0.21.7",
    edition = "2018",
    features = [
        "alloc",
        "default",
        "std",
    ],
)

rust_crate(
    name = "regex",
    crate = "regex",
    version = "1.10.5",
    edition = "2021",
    features = [
        "std",
        "unicode-bool",
    ],
    deps = [
        ":regex_automata",
        ":regex_syntax",
    ],
)

rust_crate(
    name = "tokio_macros",
    crate = "tokio-macros",
    version = "2.3.0",
    edition = "2021",
    crate_type = "proc-macro",
    deps = [
        ":proc_macro2",
        ":quote",
        ":syn",
    ],
)

rust_crate(
    name = "prost_derive",
    crate = "prost-derive",
    version = "0.12.6",
    edition = "2021",
    crate_type = "proc-macro",
    deps = [
        ":anyhow",
        ":itertools",
        ":proc_macro2",
        ":quote",
        ":syn",
    ],
)

rust_crate(
    name = "tracing_attributes",
    crate = "tracing-attributes",
    version = "0.1.27",
    edition = "2018",
    crate_type = "proc-macro",
    deps = [
        ":proc_macro2",
        ":quote",
        ":syn",
    ],
)

rust_crate(
    name = "pin_project_internal",
    crate = "pin-project-internal",
    version = "1.1.5",
    edition = "2021",
    crate_type = "proc-macro",
    deps = [
        ":proc_macro2",
        ":quote",
        ":syn",
    ],
    warns = [
        "unreachable_pub",
        "clippy::undocumented_unsafe_blocks",
        "clippy::transmute_undefined_repr",
        "clippy::trailing_empty_array",
        "single_use_lifetimes",
        "rust_2018_idioms",
        "clippy::pedantic",
        "non_ascii_idents",
        "clippy::inline_asm_x86_att_syntax",
        "improper_ctypes_definitions",
        "improper_ctypes",
        "clippy::default_union_representation",
        "clippy::as_ptr_cast_mut",
        "clippy::all",
    ],
    allows = [
        "clippy::type_complexity",
        "clippy::too_many_lines",
        "clippy::too_many_arguments",
        "clippy::struct_field_names",
        "clippy::struct_excessive_bools",
        "clippy::single_match_else",
        "clippy::single_match",
        "clippy::similar_names",
        "clippy::module_name_repetitions",
        "clippy::missing_errors_doc",
        "clippy::manual_range_contains",
        "clippy::manual_assert",
        "clippy::float_cmp",
        "clippy::doc_markdown",
        "clippy::declare_interior_mutable_const",
        "clippy::borrow_as_ptr",
        "clippy::bool_assert_comparison",
    ],
)

rust_crate(
    name = "async_trait",
    crate = "async-trait",
    version = "0.1.80",
    edition = "2021",
    crate_type = "proc-macro",
    deps = [
        ":proc_macro2",
        ":quote",
        ":syn",
    ],
)

rust_crate(
    name = "axum_core",
    crate = "axum-core",
    version = "0.3.4",
    edition = "2021",
    deps = [
        ":async_trait",
        ":bytes",
        ":futures_util",
        ":http",
        ":http_body",
        ":mime",
        ":tower_layer",
        ":tower_service",
    ],
    build_root = "build.rs",
    build_deps = [
        ":rustversion",
    ],
)

rust_crate(
    name = "async_stream",
    crate = "async-stream",
    version = "0.3.5",
    edition = "2018",
    crate_type = "lib",
    deps = [
        ":async_stream_impl",
        ":futures_core",
        ":pin_project_lite",
    ],
)

rust_crate(
    name = "async_stream_impl",
    crate = "async-stream-impl",
    version = "0.3.5",
    edition = "2018",
    crate_type = "proc-macro",
    deps = [
        ":proc_macro2",
        ":quote",
        ":syn",
    ],
)

rust_crate(
    name = "tokio",
    crate = "tokio",
    version = "1.38.0",
    edition = "2021",
    crate_type = "lib",
    features = [
        "bytes", "default", "fs", "full", "io-std", "io-util", "libc", "macros", "mio",
        "net", "num_cpus", "parking_lot", "process", "rt", "rt-multi-thread", "signal",
        "signal-hook-registry", "socket2", "sync", "time", "tokio-macros",
    ],
    deps = [
        ":bytes",
        ":libc",
        ":mio",
        ":num_cpus",
        ":parking_lot",
        ":pin_project_lite",
        ":signal_hook_registry",
        ":socket2",
        ":tokio_macros",
    ],
)

rust_crate(
    name = "pin_project",
    crate = "pin-project",
    version = "1.1.5",
    edition = "2021",
    crate_type = "lib",
    warns = [
        "unreachable_pub", "clippy::undocumented_unsafe_blocks", "clippy::transmute_undefined_repr",
        "clippy::trailing_empty_array", "single_use_lifetimes", "rust_2018_idioms", "clippy::pedantic",
        "non_ascii_idents", "clippy::inline_asm_x86_att_syntax", "improper_ctypes_definitions",
        "improper_ctypes", "clippy::default_union_representation", "clippy::as_ptr_cast_mut",
        "clippy::all",
    ],
    allows = [
        "clippy::type_complexity", "clippy::too_many_lines", "clippy::too_many_arguments",
        "clippy::struct_field_names", "clippy::struct_excessive_bools", "clippy::single_match_else",
        "clippy::single_match", "clippy::similar_names", "clippy::module_name_repetitions",
        "clippy::missing_errors_doc", "clippy::manual_range_contains", "clippy::manual_assert",
        "clippy::float_cmp", "clippy::doc_markdown", "clippy::declare_interior_mutable_const",
        "clippy::borrow_as_ptr", "clippy::bool_assert_comparison",
    ],
    deps = [
        ":pin_project_internal",
    ],
)

rust_crate(
    name = "tracing",
    crate = "tracing",
    version = "0.1.40",
    edition = "2018",
    crate_type = "lib",
    features = [
        "attributes", "default", "std", "tracing-attributes",
    ],
    deps = [
        ":pin_project_lite",
        ":tracing_attributes",
        ":tracing_core",
    ],
)

rust_crate(
    name = "prost",
    crate = "prost",
    version = "0.12.6",
    edition = "2021",
    crate_type = "lib",
    features = [
        "derive", "prost-derive", "std",
    ],
    deps = [
        ":bytes",
        ":prost_derive",
    ],
)

rust_crate(
    name = "prost_types",
    crate = "prost-types",
    version = "0.12.6",
    edition = "2021",
    crate_type = "lib",
    deps = [
        ":prost",
    ],
)

rust_crate(
    name = "prost_build",
    crate = "prost-build",
    version = "0.12.6",
    edition = "2021",
    crate_type = "lib",
    features = [
        "default", "format",
    ],
    deps = [
        ":bytes",
        ":heck",
        ":itertools",
        ":log",
        ":multimap",
        ":once_cell",
        ":petgraph",
        ":prettyplease",
        ":prost",
        ":prost_types",
        ":regex",
        ":syn",
        ":tempfile",
    ],
)

rust_crate(
    name = "tonic_build",
    crate = "tonic-build",
    version = "0.11.0",
    edition = "2021",
    crate_type = "lib",
    features = [
        "default", "prost", "prost-build", "transport",
    ],
    deps = [
        ":prettyplease",
        ":proc_macro2",
        ":prost_build",
        ":quote",
        ":syn",
    ],
)

rust_crate(
    name = "tokio_util",
    crate = "tokio-util",
    version = "0.7.11",
    edition = "2021",
    crate_type = "lib",
    features = [
        "codec", "default", "io",
    ],
    deps = [
        ":bytes",
        ":futures_core",
        ":futures_sink",
        ":pin_project_lite",
        ":tokio",
    ],
)

rust_crate(
    name = "tokio_io_timeout",
    crate = "tokio-io-timeout",
    version = "1.2.0",
    edition = "2018",
    crate_type = "lib",
    deps = [
        ":pin_project_lite",
        ":tokio",
    ],
)

rust_crate(
    name = "tokio_stream",
    crate = "tokio-stream",
    version = "0.1.15",
    edition = "2021",
    crate_type = "lib",
    features = [
        "default", "time",
    ],
    deps = [
        ":futures_core",
        ":pin_project_lite",
        ":tokio",
    ],
)

rust_crate(
    name = "h2",
    crate = "h2",
    version = "0.3.26",
    edition = "2018",
    crate_type = "lib",
    deps = [
        ":bytes",
        ":fnv",
        ":futures_core",
        ":futures_sink",
        ":futures_util",
        ":http",
        ":indexmap",
        ":slab",
        ":tokio",
        ":tokio_util",
        ":tracing",
    ],
)

rust_crate(
    name = "tower",
    crate = "tower",
    version = "0.4.13",
    edition = "2018",
    crate_type = "lib",
    features = [
        "__common", "balance", "buffer", "discover", "futures-core", "futures-util", "indexmap",
        "limit", "load", "make", "pin-project", "pin-project-lite", "rand", "ready-cache", "slab",
        "timeout", "tokio", "tokio-util", "tracing", "util",
    ],
    deps = {
        "futures_core": ":futures_core",
        "futures_util": ":futures_util",
        # https://github.com/indexmap-rs/indexmap/issues/289
        #"indexmap": ":indexmap.1.9.3",
        "indexmap": ":indexmap.1.2.0",
        "pin_project": ":pin_project",
        "pin_project_lite": ":pin_project_lite",
        "rand": ":rand",
        "slab": ":slab",
        "tokio": ":tokio",
        "tokio_util": ":tokio_util",
        "tower_layer": ":tower_layer",
        "tower_service": ":tower_service",
        "tracing": ":tracing",
    },
)

rust_crate(
    name = "hyper",
    crate = "hyper",
    version = "0.14.29",
    edition = "2018",
    crate_type = "lib",
    features = [
        "client", "default", "full", "h2", "http1", "http2", "runtime", "server", "socket2",
        "stream", "tcp",
    ],
    deps = [
        ":bytes",
        ":futures_channel",
        ":futures_core",
        ":futures_util",
        ":h2",
        ":http",
        ":http_body",
        ":httparse",
        ":httpdate",
        ":itoa",
        ":pin_project_lite",
        ":socket2",
        ":tokio",
        ":tower_service",
        ":tracing",
        ":want",
    ],
)

rust_crate(
    name = "axum",
    crate = "axum",
    version = "0.6.20",
    edition = "2021",
    crate_type = "lib",
    deps = {
        "async_trait": ":async_trait",
        "axum_core": ":axum_core",
        "bitflags": ":bitflags.1.3.2",
        "bytes": ":bytes",
        "futures_util": ":futures_util",
        "http": ":http",
        "http_body": ":http_body",
        "hyper": ":hyper",
        "itoa": ":itoa",
        "matchit": ":matchit",
        "memchr": ":memchr",
        "mime": ":mime",
        "percent_encoding": ":percent_encoding",
        "pin_project_lite": ":pin_project_lite",
        "serde": ":serde",
        "sync_wrapper": ":sync_wrapper",
        "tower": ":tower",
        "tower_layer": ":tower_layer",
        "tower_service": ":tower_service",
    },
    build_root = "build.rs",
    build_deps = [
        ":rustversion",
    ],
)

rust_crate(
    name = "hyper_timeout",
    crate = "hyper-timeout",
    version = "0.4.1",
    edition = "2018",
    crate_type = "lib",
    deps = [
        ":hyper",
        ":pin_project_lite",
        ":tokio",
        ":tokio_io_timeout",
    ],
)

rust_crate(
    name = "tonic",
    crate = "tonic",
    version = "0.11.0",
    edition = "2021",
    crate_type = "lib",
    features = [
        "channel", "codegen", "default", "prost", "transport",
    ],
    deps = [
        ":async_stream",
        ":async_trait",
        ":axum",
        ":base64",
        ":bytes",
        ":h2",
        ":http",
        ":http_body",
        ":hyper",
        ":hyper_timeout",
        ":percent_encoding",
        ":pin_project",
        ":prost",
        ":tokio",
        ":tokio_stream",
        ":tower",
        ":tower_layer",
        ":tower_service",
        ":tracing",
    ],
    compile_env = {
        "CARGO_PKG_VERSION": "0.1.0",
    },
)

#### rust-proto examples ####

rust_crate(
    name = "once_cell.1.10.0",
    crate = "once_cell",
    edition = "2021",
    version = "1.10.0",
)

rust_crate_download(
    name = "protoc_gen_prost_dl",
    crate = "protoc-gen-prost",
    version = "0.3.1",
)

rust_crate(
    name = "protoc_gen_prost_lib",
    crate = "protoc-gen-prost",
    edition = "2021",
    download = ":protoc_gen_prost_dl",
    version = "0.3.1",
    features = [],
    deps = {
        "once_cell": ":once_cell",
        "prost": ":prost",
        "prost_build": ":prost_build",
        "prost_types": ":prost_types",
        "regex": ":regex",
    },
)

rust_crate(
    name = "protoc_gen_prost",
    crate = "protoc-gen-prost",
    edition = "2021",
    download = ":protoc_gen_prost_dl",
    version = "0.3.1",
    features = [],
    crate_type = 'bin',
    src_root = 'src/main.rs',
    deps = {
        "prost": ":prost",
        "protoc_gen_prost": ":protoc_gen_prost_lib",
    },
    compile_env = {
        "CARGO_PKG_VERSION": "0.1.0",
    },
)


rust_crate_download(
    name = "protoc_gen_tonic_dl",
    crate = "protoc-gen-tonic",
    version = "0.4.0",
)

rust_crate(
    name = "protoc_gen_tonic_lib",
    crate = "protoc-gen-tonic",
    download = ":protoc_gen_tonic_dl",
    edition = "2021",
    features = [],
    version = "0.4.0",
    deps = {
        "heck": ":heck",
        "prettyplease": ":prettyplease",
        "prost_build": ":prost_build",
        "prost_types": ":prost_types",
        "prost": ":prost",
        "protoc_gen_prost": ":protoc_gen_prost_lib",
        "regex": ":regex",
        "syn": ":syn",
        "tonic_build": ":tonic_build",
    },
)

rust_crate(
    name = "protoc_gen_tonic",
    compile_env = {
        "CARGO_PKG_VERSION": "0.1.0",
    },
    crate = "protoc-gen-tonic",
    crate_type = "bin",
    download = ":protoc_gen_tonic_dl",
    edition = "2021",
    features = [],
    src_root = "src/main.rs",
    version = "0.4.0",
    deps = {
        "prost": ":prost",
        "protoc_gen_prost": ":protoc_gen_prost_lib",
        "protoc_gen_tonic": ":protoc_gen_tonic_lib",
    },
)
