subinclude("//build_defs:rust")

rust_library(
    name = "math",
    root = "src/lib.rs",
    modules = [
        "src/add.rs",
        "src/sub.rs",
    ],
)

# rust_binary(
#     name = "main",
#     main = "src/main.rs",
#     deps = [
#         ":math",
#     ],
# )

rust_test(
    name = 'test',
    root = 'src/lib.rs',
    modules = [
        "src/add.rs",
        "src/sub.rs",
    ],  
)
