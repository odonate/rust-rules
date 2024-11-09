rust_library(
    name = "service",
    edition = "2021",
    modules = [
        "src/service/mod.rs",
        "src/service/service.rs",
    ],
    root = "src/lib.rs",
    deps = [
        "//examples/rust_proto/proto:api",
        "//third_party/rust:async_trait",
        "//third_party/rust:prost",
        "//third_party/rust:tonic",
    ],
    visibility = ["PUBLIC"],
)
