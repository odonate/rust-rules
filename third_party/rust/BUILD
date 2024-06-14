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
    build_script = "build.rs",
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
