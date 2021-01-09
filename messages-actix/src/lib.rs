#[macro_use]
extern crate actix_web;

use std::cell::Cell;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use actix_web::{App, HttpServer, middleware, Result, web};
use serde::{Serialize, Deserialize};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct MessageApp {
    port: u16,
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    message: Vec<String>,
}

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Deserialize)]
struct PostInput {
    message: String
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        message: ms.to_owned(),
    }))
}

#[post("/clear")]
fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    ms.clear();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        message: vec![],
    }))
}


fn post(msg: web::Json<PostInput>, state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    ms.push(msg.message.clone());

    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count,
        message: msg.message.clone(),
    }))
}


impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server 127.0.0.1:{}", self.port);

        let messages = Arc::new(Mutex::new(vec![]));
        HttpServer::new(move || {
            App::new()
                .data(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                })
                .wrap(middleware::Logger::default())
                .service(index)
                .service(web::resource("/send")
                    .data(web::JsonConfig::default().limit(4096))
                    .route(web::post().to(post))
                )
                .service(clear)
        })
            .bind(("127.0.0.1", self.port))?
            .workers(8)
            .run()
    }
}