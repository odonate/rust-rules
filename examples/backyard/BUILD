subinclude("//build_defs:rust")

rust_library(
    name = "garden",
    root = "src/lib.rs",
    modules = [
        "src/patch.rs",
        "src/patch/vegetables.rs",
        "src/greenhouse.rs",
        "src/greenhouse/vegetables.rs",
    ],
)

rust_binary(
    name = "backyard",
    main = "src/main.rs",
    deps = [
        ":garden",
    ],
)
