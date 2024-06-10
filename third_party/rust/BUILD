subinclude("//build_defs:rust")

rust_toolchain(
    name = "toolchain",
    hashes = ["03d89cfdd013996f9911165ffb5790c5fd16f059"],
    version = "1.78.0",
    visibility = ["PUBLIC"],
)

rust_crate(
    name="cfg_if",
    crate="cfg-if",
    version="1.0.0",
    edition="2018",
)

rust_crate(
    name="libc",
    crate="libc",
    version="0.2.154",
    edition="2015",
)

rust_crate(
    name="wasi",
    crate="wasi",
    version="0.11.0+wasi-snapshot-preview1",
    edition="2018",
)

rust_crate(
    name="getrandom",
    crate="getrandom",
    version="0.2.15",
    deps=[
        ":cfg_if",
        ":libc",
        ":wasi",
    ],
    features=["std"],
    edition="2018",
)

rust_crate(
    name="rand_core",
    crate="rand_core",
    version="0.6.4",
    # features=["std"],
    # deps = [":getrandom"],
    edition="2018",
)

#### RUST VERSIONS ARE A MESS ####
# rust_crate(
#     name="ppv_lite86",
#     crate="ppv-lite86",
#     version="0.2.1",
#     edition="2018",
# )

# rust_crate(
#     name="c2_chacha",
#     crate="c2-chacha",
#     version="0.2.2",
#     deps = [
#         ":ppv_lite86",
#     ],
#     features=["simd"],
#     edition="2015",
# )

# rust_crate(
#     name="rand_chacha",
#     crate="rand_chacha",
#     version="0.2.1",
#     deps=[
#         ":c2_chacha",
#         ":rand_core",
#     ],
#     features=["std"],
#     edition="2018",
# )

# rust_crate(
#     name="rand_hc",
#     crate="rand_hc",
#     version="0.2.0",
#     deps=[
#         ":rand_core",
#     ],
#     features=["std"],
#     edition="2018",
# )

# rust_crate(
#     name="rand",
#     crate="rand",
#     version="0.7.3",
#     deps=[
#         ":rand_chacha",
#         ":rand_core",
#         ":rand_hc",
#     ],
#     edition="2018",
# )
