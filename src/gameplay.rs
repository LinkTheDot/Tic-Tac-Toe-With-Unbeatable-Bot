use crate::coordinate_methods::*;
use crate::gameboard::*;
use std::error::Error;
use std::io;

#[derive(PartialEq, Debug)]
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

  pub fn check_if_win(&mut self) -> bool {
    self
      .game_board
      .coordinates_connected_to_three_in_a_row(&self.game_board.last_modified_tile)
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
  let mut gameconfig = GameConfig::new();

  while gameconfig.game_board.tiles_covered < 9 {
    gameconfig.game_board.print_board();
    println!();

    if gameconfig.player_turn {
      let selected_tile = match parse_player_input() {
        Ok(x) => x,
        Err(_) => continue,
      };

      println!("selected tile = {:?}", selected_tile);

      if gameconfig.game_board.tiles[selected_tile.0][selected_tile.1].board_state
        == BoardStates::Empty
      {
        gameconfig.game_board.tiles[selected_tile.0][selected_tile.1].board_state =
          gameconfig.player_symbol.clone();

        gameconfig.game_board.tiles_covered += 1;
        gameconfig.player_turn = false;
      }

      if gameconfig.check_if_win() {
        gameconfig.game_state = gameconfig.player_symbol_to_game_state();
        break;
      }
    } else {
      // insert bot code

      //temp stuff until i do add it
      gameconfig.player_turn = true;
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
        return Ok(((num - 1) / GRID_SIZE, (num - 1) % GRID_SIZE));
      }
    }
  } else if player_input.trim().len() == 2 {
    let coord_1 = match player_input[0..1].to_lowercase().trim() {
      "a" => 0,
      "b" => 1,
      "c" => 2,
      _ => return Err(Box::from("incorrect input")),
    };

    if let Ok(num) = player_input[1..2].trim().parse::<usize>() {
      if num != 0 && num <= GRID_SIZE {
        return Ok((coord_1, num - 1));
      }
    }
  }

  Err(Box::from("incorrect input"))
}
