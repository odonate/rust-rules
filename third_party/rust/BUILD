subinclude("//build_defs:rust")

rust_toolchain(
    name = "toolchain",
    hashes = ["03d89cfdd013996f9911165ffb5790c5fd16f059"],
    version = "1.78.0",
    visibility = ["PUBLIC"],
)

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
