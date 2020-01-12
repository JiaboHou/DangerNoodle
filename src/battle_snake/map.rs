use std::collections::hash_map::HashMap;
use std::convert::TryInto;
use std::fmt;

use crate::battle_snake::structs::{POSSIBLE_MOVES, GameEnvironment, Move, Point, Snake};

#[derive(Clone)]
pub enum SpaceType {
  Snake,
  Food,
}

#[derive(Clone)]
pub struct Space {
  space_type: SpaceType,
  snake_id: Option<String>,
  next_body: Option<Point>,
}

#[allow(dead_code)]
pub struct Map<'a> {
  grid: Vec<Option<Space>>,
  height: u8,
  width: u8,
  // food: Vec<Point>,
  you: String,
  snakes: HashMap<String, &'a Snake>,
}

impl<'a> fmt::Display for Map<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    writeln!(f, "Grid: {} by {}", self.width, self.height)?;

    for row in self.grid.chunks(self.width as usize) {
      write!(f, "|")?;
      for space in row.iter() {
        let _ = match space {
          None => write!(f, "X"),
          Some(Space { space_type: SpaceType::Snake, snake_id: _, next_body: None, }) => write!(f, "T"),
          Some(Space { space_type: SpaceType::Snake, snake_id: _, next_body: Some(_), }) => write!(f, "S"),
          Some(Space { space_type: SpaceType::Food, snake_id: _, next_body: _, }) => write!(f, "F"),
        };
      }
      writeln!(f, "|")?;
    }
    write!(f, "\n")
  }
}

// static EMPTY_SPACE: Space = Space { space_type: SpaceType::Empty, snake_id: None, next_body: None };

// TODO: verify that the biggest map is 15x15
fn point_to_index(width: u8, point: &Point) -> u8 {
  point.y * width + point.x
}

pub fn generate_map<'a>(game_environment: &'a GameEnvironment) -> Box::<Map<'a>> {
  let height = game_environment.board.height;
  let width = game_environment.board.width;
  let capacity: usize = (height * width).try_into().unwrap();
  let mut grid = vec![None; capacity];
  let mut snake_hash = HashMap::new();

  for food in &game_environment.board.food {
    let grid_index: usize = (point_to_index(width, food)).try_into().unwrap();
    grid[grid_index] = Some(Space { space_type: SpaceType::Food, snake_id: None, next_body: None });
  }

  for snake in &game_environment.board.snakes {
    for (body_part_ind, body_part_val) in snake.body.iter().enumerate() {
      let grid_index: usize = (point_to_index(width, body_part_val)).try_into().unwrap();
      let next: Option<Point> = if (body_part_ind + 1) >= snake.body.len() {
        None
      } else {
        Some(snake.body[body_part_ind + 1].clone())
      };
      grid[grid_index] = Some(Space { space_type: SpaceType::Snake, snake_id: Some(snake.id.clone()), next_body: next });
    }
    snake_hash.insert(snake.id.clone(), snake);
  }

  let map = Box::<Map>::new(Map {
    grid,
    height,
    width,
    snakes: snake_hash,
    you: game_environment.you.id.clone(),
  });
  return map;
}

fn is_valid_move(map: &Map, start_point: &Point, r#move: &Move) -> bool {
  let proposed_point;
  let passes_border = match r#move {
    Move::Left => {
      proposed_point = Point { x: start_point.x - 1, y: start_point.y };
      start_point.x > 0
    },
    Move::Right => {
      proposed_point = Point { x: start_point.x + 1, y: start_point.y };
      (start_point.x + 1) < map.width
    },
    Move::Up => {
      proposed_point = Point { x: start_point.x, y: start_point.y - 1 };
      start_point.y > 0
    },
    Move::Down => {
      proposed_point = Point { x: start_point.x, y: start_point.y + 1 };
      (start_point.y + 1) < map.height
    },
  };

  if !passes_border {
    return false;
  }

  let space_does_not_contain_snake = match map.grid[point_to_index(map.width, &proposed_point) as usize] {
    Some(Space { space_type: SpaceType::Food, snake_id: _, next_body: _, }) | None => true,
    Some(Space { space_type: SpaceType::Snake, snake_id: _, next_body: _, }) => false,
  };

  space_does_not_contain_snake
}

// Return of Vec of static lifetime Moves, as those values are referencing POSSIBLE_MOVES, an array with a static lifetime
pub fn get_valid_moves(map: &Map, snake: &Snake) -> Vec<&'static Move> {
  POSSIBLE_MOVES
    .iter()
    .filter(|m| is_valid_move(map, &snake.body[0], *m))
    .collect()
}
