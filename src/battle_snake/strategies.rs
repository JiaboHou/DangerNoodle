use rand::Rng;

use crate::battle_snake::structs::{POSSIBLE_MOVES, Point, GameEnvironment, Move, Snake};
use crate::battle_snake::map::{generate_map, get_valid_moves, move_toward};

pub fn is_occupied(snakes: &Vec<Snake>, location: &Point) -> bool {
  for snake in snakes.iter() {
    for body_part in snake.body.iter() {
      // println!("is_occupied: body_part x: {}, y: {}", body_part.x, body_part.y);
      // println!("is_occupied: location x: {}, y: {}", location.x, location.y);
      if body_part.x == location.x && body_part.y == location.y {
        return true;
      }
    }
  }
  return false;
}

pub fn is_out_of_bounds(width: &u8, height: &u8, location: &Point) -> bool {
  let out_of_bounds = (location.x >= *width) || (location.y >= *height);
  println!("out of bounds {}", out_of_bounds);
  return out_of_bounds;
}

pub fn is_valid_move(data: &GameEnvironment, movement: &Move) -> bool {
  let head_position = &data.you.body[0];

  let new_position = match movement {
    Move::Left => {
      if head_position.x == 0 {
        return false;
      } else {
        Point { x: head_position.x - 1, y: head_position.y }
      }
    }
    Move::Right => Point { x: head_position.x + 1, y: head_position.y },
    Move::Up => {
      if head_position.y == 0 {
        return false;
      } else {
        Point { x: head_position.x, y: head_position.y - 1 }
      }
    }
    Move::Down => Point { x: head_position.x, y: head_position.y + 1 },
  };

  println!("is_valid_move: x: {}, y: {}", new_position.x, new_position.y);
  let out_of_bounds: bool = is_out_of_bounds(&data.board.width, &data.board.height, &new_position);
  if out_of_bounds {
    return !out_of_bounds;
  }

  return !is_occupied(&data.board.snakes, &new_position);
}

// Randomly move in any direction that is not occupied by a snake
// including yourself, and walls
#[allow(dead_code)]
pub fn random_v0(data: GameEnvironment) -> Move {
  println!("calculating valid moves");
  let valid_moves: Vec<&Move> = POSSIBLE_MOVES
    .iter()
    .filter(|m| is_valid_move(&data, *m))
    .collect();

  println!("valid moves: {}", valid_moves.len());

  let movement_number: usize = rand::thread_rng().gen_range(0, valid_moves.len());

  return valid_moves[movement_number].clone();
}

#[allow(dead_code)]
pub fn random_v1(data: GameEnvironment) -> Move {
  // 1. Generate grid
  let map = generate_map(&data);

  // 2. Cull invalid moves
  let valid_moves = get_valid_moves(&map, &data.you);

  // 3. Randomly choose between valid moves
  let movement_number: usize = rand::thread_rng().gen_range(0, valid_moves.len());

  return valid_moves[movement_number].clone();
}

#[allow(dead_code)]
pub fn chase_tail(data: GameEnvironment) -> Move {
  // 1. Generate grid
  let map = generate_map(&data);

  // 2. Cull invalid moves
  let valid_moves = get_valid_moves(&map, &data.you);

  println!("Valid Moves: {:?}", valid_moves);

  // 3. Choose move that goes toward your own tail
  let my_body = &map.snakes[&map.you].body;
  let my_head = my_body[0].clone();
  let my_tail = my_body[my_body.len() - 1].clone();
  let tail_moves = move_toward(&my_head, &my_tail, &valid_moves);
  println!("Tail Moves: {:?}", tail_moves);

  // 4. Randomly choose between moves that chase your own tail
  let movement_number: usize = rand::thread_rng().gen_range(0, tail_moves.len());

  return valid_moves[movement_number].clone();
}

// TODO: evaluate moves on a lifetime count basis?