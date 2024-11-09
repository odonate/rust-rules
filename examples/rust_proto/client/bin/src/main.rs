use std::error::Error;

use api::api::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let _ = tonic::Request::new(HelloRequest {
        person: "World".to_string(),
    });
    Ok(())
}
