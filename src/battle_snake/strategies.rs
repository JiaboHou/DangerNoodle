use rand::Rng;

use crate::battle_snake::structs::{ BodyPart, GameEnvironment, Move, Snake, Point };

static possible_moves: [Move; 4] = [Move::Left, Move::Right, Move::Up, Move::Down];

// fn generate_grid() {

// }

// fn get_valid_moves() -> Move {

// }

pub fn is_occupied(snakes: &Vec<Snake>, location: BodyPart) -> bool {
  let mut occupied: bool = false;
  for snake in snakes.iter() {
    for body_part in snake.body.iter() {
      println!("is_occupied: body_part x: {}, y: {}", body_part.x, body_part.y);
      println!("is_occupied: location x: {}, y: {}", location.x, location.y);
      occupied = occupied || match body_part {
        BodyPart { x, y } => *x == location.x && *y == location.y,
        _ => false,
      };

      if occupied {
        return occupied;
      }
    }
  }
  return false;
}

pub fn is_valid_move(data: &GameEnvironment, movement: &Move) -> bool {
  let head_position = &data.you.body[0];

  let new_position = match movement {
    Move::Left => BodyPart { x: head_position.x - 1, y: head_position.y },
    Move::Right => BodyPart { x: head_position.x + 1, y: head_position.y },
    Move::Up => BodyPart { x: head_position.x, y: head_position.y - 1 },
    Move::Down => BodyPart { x: head_position.x, y: head_position.y + 1 },
  };

  println!("is_valid_move: x: {}, y: {}", new_position.x, new_position.y);

  return !is_occupied(&data.board.snakes, new_position);
}

pub fn random_v0(data: GameEnvironment) -> Move {
  println!("calculating valid moves");
  let valid_moves: Vec<&Move> = possible_moves
    .iter()
    .filter(|m| is_valid_move(&data, *m))
    .collect();

  println!("valid moves: {}", valid_moves.len());

  let movement_number: usize = rand::thread_rng().gen_range(0, valid_moves.len());

  return *(valid_moves[movement_number]);
}

pub fn random_v1(data: GameEnvironment) -> Move {
  // 1. Generate grid

  // 2. Cull invalid moves

  // 3. Score move

  return Move::Left;
}
