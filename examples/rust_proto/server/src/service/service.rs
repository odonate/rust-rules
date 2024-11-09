use tonic::{Request, Response, Status};
use async_trait::async_trait;

use api::api;
use api::greeter_server::{Greeter, GreeterServer};
use api::{HelloRequest, HelloResponse};

#[derive(Default)]
pub struct MyGreeter;

impl MyGreeter {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn server(self) -> GreeterServer<Self> {
        GreeterServer::new(self)
    }
}

#[async_trait]
impl Greeter for MyGreeter {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let reply = HelloResponse {
            greeting: format!("Hello {}!", request.into_inner().person),
        };
        Ok(Response::new(reply))
    }
}
