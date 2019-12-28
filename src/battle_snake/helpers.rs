use std::collections::hash_map::HashMap;
use std::convert::TryInto;

use crate::battle_snake::structs::{Point, GameEnvironment};

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

pub struct Snake {
  locations: Vec<Point>,
  health: u8,
}

pub struct Map {
  grid: Vec<Option<Space>>,
  height: u8,
  width: u8,
  // food: Vec<Point>,
  // snakes: HashMap<String, Snake>,
}

// static EMPTY_SPACE: Space = Space { space_type: SpaceType::Empty, snake_id: None, next_body: None };

pub fn generate_map(game_environment: &GameEnvironment) -> Map {
  let height = game_environment.board.height;
  let width = game_environment.board.width;
  let capacity: usize = (height * width).try_into().unwrap();
  let mut grid = vec![None; capacity];

  for food in &game_environment.board.food {
    let grid_index: usize = (food.y * width + food.x).try_into().unwrap();
    grid[grid_index] = Some(Space { space_type: SpaceType::Food, snake_id: None, next_body: None });
  }

  for snake in &game_environment.board.snakes {
    for (body_part_ind, body_part_val) in snake.body.iter().enumerate() {
      let grid_index: usize = (body_part_val.y * width + body_part_val.x).try_into().unwrap();
      let next: Option<Point> = if body_part_ind > snake.body.len() {
        None
      } else {
        Some(snake.body[body_part_ind + 1].clone())
      };
      grid[grid_index] = Some(Space { space_type: SpaceType::Snake, snake_id: Some(snake.id.clone()), next_body: next});
    }
  }

  let map = Map { grid, height, width };
  return map;
}