#[macro_use]
extern crate actix_web;

use actix_web::{App, HttpRequest, HttpServer, middleware, Result, web};
use serde::Serialize;

pub struct MessageApp {
    port: u16,
}

#[derive(Serialize)]
struct IndexResponse {
    message: String
}

#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
    .headers()
    .get("hello")
    .and_then(|v| v.to_str().ok())
    .unwrap_or_else(|| "world");

    Ok(web::Json(IndexResponse {
        message: hello.to_owned(),
    }))
}


impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}