use actix_web::{post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,};
use listenfd::ListenFd;

use serde_json;

use std::env;

mod battle_snake_structs;

impl Responder for battle_snake_structs::StartResponse {
  type Error = Error;
  type Future = Result<HttpResponse, Error>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self)?;
    Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(body))
  }
}
impl Responder for battle_snake_structs::MoveResponse {
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
  let start_obj = battle_snake_structs::StartResponse::new(
    "#ea5b0c".to_string(),
    "safe".to_string(),
    "small-rattle".to_string(),
  );
  return start_obj;
}

#[post("/move")]
fn handler_move(request_body: web::Json<battle_snake_structs::GameEnvironment>) -> impl Responder {
  println!("POST /move");
  println!("{}", request_body.turn);

  let move_obj = battle_snake_structs::MoveResponse::new(battle_snake_structs::Move::Left);
  return move_obj;
}

#[post("/end")]
fn handler_end() -> impl Responder {
  println!("POST /end");
  HttpResponse::Ok();
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
