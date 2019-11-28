use actix_web::{post, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,};
use listenfd::ListenFd;
use serde::Serialize;
use serde_json;

use std::env;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StartConfig {
  color: &'static str,
  head_type: &'static str,
  tail_type: &'static str,
}

impl Responder for StartConfig {
  type Error = Error;
  type Future = Result<HttpResponse, Error>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self)?;
    Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(body))
  }
}

#[post("/start")]
fn handler_start() -> impl Responder {
  println!("POST /start");
  let start_obj = StartConfig {
    color: "user",
    head_type: "safe",
    tail_type: "small-rattle",
  };
  return start_obj;
}

#[post("/move")]
fn handler_move() -> impl Responder {
  println!("POST /move");
  HttpResponse::Ok().body("Hello world!");
}

#[post("/end")]
fn handler_end() -> impl Responder {
  println!("POST /end");
  HttpResponse::Ok().body("Hello world!");
}

#[post("/ping")]
fn handler_ping() -> impl Responder {
  println!("POST /ping");
  HttpResponse::Ok();
}

fn get_server_port() -> u16 {
  env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3000)
}

fn main() {
  println!("Hello, world!");

  let mut listenfd = ListenFd::from_env();
  let mut server = HttpServer::new(||
    App::new()
      .service(handler_start)
      .service(handler_move)
      .service(handler_end)
      .service(handler_ping)
  );

  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l).unwrap()
  } else {
    server.bind(("0.0.0.0", get_server_port())).unwrap()
  };

  server.run().unwrap();
}
