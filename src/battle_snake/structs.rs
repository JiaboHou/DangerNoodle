use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartResponse {
  color: String,
  head_type: String,
  tail_type: String,
}

impl StartResponse {
  pub fn new(color: String, head_type: String, tail_type: String) -> StartResponse {
    StartResponse {
      color,
      head_type,
      tail_type,
    }
  }
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Move {
  Left,
  Right,
  Up,
  Down,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveResponse {
  r#move: Move,
}

impl MoveResponse {
  pub fn new(r#move: Move) -> MoveResponse {
    MoveResponse {
      r#move,
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct Game {
  id: String,
}

#[derive(Deserialize, Debug)]
pub struct Point {
  pub x: u8,
  pub y: u8,
}

#[derive(Deserialize, Debug)]
pub struct Food {
  pub x: u8,
  pub y: u8,
}

#[derive(Deserialize, Debug)]
pub struct BodyPart{
  pub x: i8,
  pub y: i8,
}

#[derive(Deserialize, Debug)]
pub struct Snake {
  pub id: String,
  pub name: String,
  pub health: u8,
  pub body: Vec<BodyPart>,
}

#[derive(Deserialize, Debug)]
pub struct Board {
  pub height: i8,
  pub width: i8,
  pub food: Vec<Food>,
  pub snakes: Vec<Snake>,
}

#[derive(Deserialize, Debug)]
pub struct GameEnvironment {
  pub game: Game,
  pub turn: u16,
  pub board: Board,
  pub you: Snake,
}
