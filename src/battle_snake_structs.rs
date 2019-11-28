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

#[derive(Serialize)]
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
  x: u8,
  y: u8,
}

#[derive(Deserialize, Debug)]
pub struct Food {
  x: u8,
  y: u8,
}

#[derive(Deserialize, Debug)]
pub struct BodyPart{
  x: u8,
  y: u8,
}

#[derive(Deserialize, Debug)]
pub struct Snake {
  id: String,
  name: String,
  health: u8,
  body: Vec<BodyPart>,
}

#[derive(Deserialize, Debug)]
pub struct Board {
  height: u8,
  width: u8,
  food: Vec<Food>,
  snakes: Vec<Snake>,
}

#[derive(Deserialize, Debug)]
pub struct GameEnvironment {
  pub game: Game,
  pub turn: u16,
  pub board: Board,
  pub you: Snake,
}
