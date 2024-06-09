subinclude("//build_defs:rust")

filegroup(
    name = "mods",
    srcs = [
        "add.rs",
        "sub.rs",
    ],
)

rust_library(
    name = "math",
    srcs = ["lib.rs"],
    deps = [":mods"],
)

rust_binary(
    name = "guess",
    main = "main.rs",
    deps = [
        ":math",
        # "//third_party/rust:rand",
        # "//third_party/rust:rand_core",
    ],
)
