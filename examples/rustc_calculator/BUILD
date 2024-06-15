rust_binary(
    name = "rustc_calculator",
    main = "src/main.rs",
    edition = "2021",
    deps = [
        "//third_party/rust:tonic",
        "//third_party/rust:tokio",
        "//third_party/rust:prost",
    ],
    build_root = "build.rs",
    build_deps = ["//third_party/rust:tonic_build"],
)
