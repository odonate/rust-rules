subinclude("///proto//build_defs:proto")

grpc_library(
    name = "api",
    srcs = ["api.proto"],
    languages = {
        "rust": rust_grpc_language(),
    },
    visibility = ["PUBLIC"],
    deps = [
        "//third_party/rust:prost",
        "//third_party/rust:tonic",
    ],
)
