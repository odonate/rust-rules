use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("PROTOC", "/home/jack/rust-rules/plz-out/bin/third_party/proto/protoc");

    if let Ok(protoc) = env::var("PROTOC") {
        println!("PROTOC environment variable: {}", protoc);
    } else {
        println!("PROTOC environment variable is not set.");
    }
    
    tonic_build::compile_protos("proto/calculator.proto")?;

    Ok(())
}
