extern crate tokio_service;
extern crate tokio_proto;
extern crate tokio_minihttp;
extern crate futures;
extern crate num_cpus;
extern crate serde_json;

use futures::future;
use tokio_service::Service;
use tokio_proto::TcpServer;
use tokio_minihttp::{Request, Response, Http};
use serde_json::builder::ObjectBuilder;

struct Zap;

impl Service for Zap {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ok<Response, std::io::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut resp = Response::new();

        match req.path() {
            "/" => {
                let json = serde_json::to_string(
                    &ObjectBuilder::new().insert(
                            "status", "ok"
                        ).build()
                    ).unwrap();

                resp.header("Content-Type", "application/json")
                    .body(&json);
            },
            _ => {
                resp.status_code(404, "Not Found");
            }
        }

        future::ok(resp)
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut server = TcpServer::new(Http, addr);
    server.threads(num_cpus::get());
    server.serve(|| Ok(Zap));
}
