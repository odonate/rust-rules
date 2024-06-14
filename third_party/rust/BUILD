subinclude("//build_defs:rust")

rust_toolchain(
    name = "toolchain",
    hashes = ["03d89cfdd013996f9911165ffb5790c5fd16f059"],
    version = "1.78.0",
    visibility = ["PUBLIC"],
)

#### Used by examples/rustc_guessing_game ####
rust_crate(
    name="libc",
    crate="libc",
    version="0.2.155",
    build_root="build.rs",
    edition="2015",
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
    features=["alloc", "default", "getrandom", "libc", "rand_chacha", "std", "std_rng"],
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
    name = "equivalent",
    crate = "equivalent",
    edition = "2015",
    version = "1.0.1",
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
    build_root = "build.rs",
    build_deps = [":autocfg"],
    features = ["atomic_usize", "default"],
    deps = [":scopeguard"],
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
