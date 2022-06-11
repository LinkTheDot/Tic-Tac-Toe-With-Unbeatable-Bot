use crate::gameboard::*;
use rand::*;
use std::error::Error;
use std::io;

pub struct GameConfig {
  player_turn: bool,
  player_symbol: BoardState,
  game_board: BoardConfig,
  game_state: GameState,
}

impl GameConfig {
  pub fn new() -> GameConfig {
    let player_turn = rand::random::<bool>();
    let player_symbol = if player_turn {
      BoardState::X
    } else {
      BoardState::O
    };
    let game_board = BoardConfig::new();
    let game_state = GameState::OnGoing;

    GameConfig {
      player_turn,
      player_symbol,
      game_board,
      game_state,
    }
  }
}

#[derive(PartialEq)]
pub enum GameState {
  XWon,
  YWon,
  Draw,
  OnGoing,
}

pub type Coordinates = (usize, usize);

pub fn run_gameplay() -> Result<(), Box<dyn Error>> {
  let mut game_config = GameConfig::new();

  while game_config.game_board.tiles_covered < 9 {
    game_config.game_board.print_board();
    println!();

    if game_config.player_turn {
      println!("Select a tile 1-9 or a1, b2, c3.");

      let mut player_input = String::new();
      io::stdin().read_line(&mut player_input).unwrap();

      let selected_tile = match parse_player_input(player_input) {
        Some(x) => x,
        None => continue,
      };

      println!("selected tile = {:?}", selected_tile);

      if game_config.game_board.tiles[selected_tile.0][selected_tile.1].board_state
        == BoardState::Empty
      {
        game_config.game_board.tiles[selected_tile.0][selected_tile.1].board_state =
          game_config.player_symbol.clone();

        game_config.game_board.tiles_covered += 1;
      } else {
        continue;
      }
    } else {
      // insert bot code

      //temp stuff until i do add it
      game_config.player_turn = true;
      continue;
    }
  }

  Ok(())
}

pub fn parse_player_input(player_input: String) -> Option<Coordinates> {
  if player_input.trim().len() == 1 {
    if let Ok(num) = player_input.trim().parse::<usize>() {
      if num == 0 {
        None
      } else {
        Some(((num - 1) / 3, (num - 1) % 3))
      }
    } else {
      None
    }
  } else if player_input.trim().len() == 2 {
    match player_input.as_str().to_lowercase().trim() {
      "a1" => Some((0, 0)),
      "a2" => Some((0, 1)),
      "a3" => Some((0, 2)),
      "b1" => Some((1, 0)),
      "b2" => Some((1, 1)),
      "b3" => Some((1, 2)),
      "c1" => Some((2, 0)),
      "c2" => Some((2, 1)),
      "c3" => Some((2, 2)),
      _ => None,
    }
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn testing_coordinate_conversion() {
    let num = 5;

    assert_eq!(((num - 1) / 3, (num - 1) % 3), (1, 1))
  }
}
