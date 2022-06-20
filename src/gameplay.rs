use crate::coordinate_methods::*;
use crate::gameboard::*;
use rand::*;
use std::error::Error;
use std::io;

pub struct GameConfig {
  pub player_turn: bool,
  pub player_symbol: BoardStates,
  pub game_board: BoardConfig,
  pub game_state: GameState,
}

impl GameConfig {
  pub fn new() -> GameConfig {
    let player_turn = rand::random::<bool>();
    let game_board = BoardConfig::new();
    let game_state = GameState::OnGoing;
    let player_symbol = if player_turn {
      BoardStates::X
    } else {
      BoardStates::O
    };

    GameConfig {
      player_turn,
      player_symbol,
      game_board,
      game_state,
    }
  }

  pub fn check_if_win(&mut self, latest_tile: Coordinates) -> bool {
    if self
      .game_board
      .coordinates_connected_to_three_in_a_row(latest_tile)
    {
      self.game_state = self.player_symbol_to_game_state();
      true
    } else {
      false
    }
  }

  pub fn player_symbol_to_game_state(&mut self) -> GameState {
    match self.player_symbol {
      BoardStates::X => GameState::XWon,
      BoardStates::O => GameState::OWon,
      _ => {
        if self.game_board.tiles_covered == 9 {
          GameState::Draw
        } else {
          GameState::OnGoing
        }
      }
    }
  }
}

#[derive(PartialEq, Clone, Debug)]
pub enum GameState {
  XWon,
  OWon,
  Draw,
  OnGoing,
}

pub fn run_gameplay() -> Result<(), Box<dyn Error>> {
  let mut game_config = GameConfig::new();

  while game_config.game_board.tiles_covered < 9 {
    game_config.game_board.print_board();
    println!();

    if game_config.player_turn {
      let selected_tile = match parse_player_input() {
        Ok(x) => x,
        Err(y) => continue,
      };

      println!("selected tile = {:?}", selected_tile);

      if game_config.game_board.tiles[selected_tile.0][selected_tile.1].board_state
        == BoardStates::Empty
      {
        game_config.game_board.tiles[selected_tile.0][selected_tile.1].board_state =
          game_config.player_symbol.clone();

        game_config.game_board.tiles_covered += 1;
        game_config.player_turn = false;
      }

      if game_config.check_if_win(selected_tile) {
        break;
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

pub fn parse_player_input() -> Result<Coordinates, Box<dyn Error>> {
  println!("Select a tile 1-9 or a1, b2, c3, etc.");

  let mut player_input = String::new();
  io::stdin().read_line(&mut player_input).unwrap();

  if player_input.trim().len() == 1 {
    if let Ok(num) = player_input.trim().parse::<usize>() {
      if num != 0 {
        return Ok(((num - 1) / 3, (num - 1) % 3));
      }
    }
  } else if player_input.trim().len() == 2 {
    let mut coord_1 = match player_input[0..1].to_lowercase().trim() {
      "a" => 0,
      "b" => 1,
      "c" => 2,
      _ => return Err(Box::from("incorrect input")),
    };

    if let Ok(num) = player_input[1..2].trim().parse::<usize>() {
      if num != 0 && num <= 3 {
        return Ok((coord_1, num - 1));
      }
    }
  }

  Err(Box::from("incorrect input"))
}

#[cfg(test)]
mod gameplay_tests {
  use super::*;

  #[test]
  fn testing_coordinate_conversion() {
    let num = 5;

    assert_eq!(((num - 1) / 3, (num - 1) % 3), (1, 1))
  }

  #[test]
  fn check_if_win_logic_works() {
    let mut game_config = GameConfig::new();
    let latest_tile_true = (0, 0);
    let latest_tile_false = (1, 0); // edge that's same but not in row
    let latest_tile_center = (1, 1);

    //for latest_tile_true
    game_config.game_board.tiles[0][0].board_state = BoardStates::X;
    game_config.game_board.tiles[0][1].board_state = BoardStates::X;
    game_config.game_board.tiles[0][2].board_state = BoardStates::X;

    //for latest_tile_false
    game_config.game_board.tiles[1][0].board_state = BoardStates::X;

    //center to edge/corner
    game_config.game_board.tiles[1][1].board_state = BoardStates::O;
    game_config.game_board.tiles[2][0].board_state = BoardStates::O;
    game_config.game_board.tiles[2][1].board_state = BoardStates::O;

    println!(
      "{:?}",
      game_config
        .game_board
        .get_board_position(&latest_tile_center)
    );

    assert_eq!(game_config.check_if_win(latest_tile_true), true);
    assert_eq!(game_config.check_if_win(latest_tile_false), false);
    assert_eq!(game_config.check_if_win(latest_tile_center), false);
  }
}
