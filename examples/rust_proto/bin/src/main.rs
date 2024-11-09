use api;
use service::MyGreeter;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = MyGreeter::new();
    
    Ok(())
}
