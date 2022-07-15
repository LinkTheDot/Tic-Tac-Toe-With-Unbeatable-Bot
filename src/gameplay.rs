use crate::bot::*;
use crate::coordinate_methods::*;
use crate::gameboard::*;
use std::error::Error;
use std::io;

const DEBUG_COORDS: Coordinates = (10, 10);

#[derive(PartialEq, Debug)]
pub struct GameConfig {
  pub player_turn: bool,
  pub player_symbol: BoardStates,
  pub gameboard: BoardConfig,
  pub gamestate: GameState,
  pub bot: Bot,
}

impl GameConfig {
  pub fn new() -> GameConfig {
    let player_turn = rand::random::<bool>();
    let gameboard = BoardConfig::new();
    let gamestate = GameState::OnGoing;
    let mut bot = Bot::new();
    let player_symbol = if player_turn {
      bot.bot_symbol = BoardStates::O;
      BoardStates::X
    } else {
      bot.bot_symbol = BoardStates::X;
      BoardStates::O
    };

    GameConfig {
      player_turn,
      player_symbol,
      gameboard,
      gamestate,
      bot,
    }
  }

  pub fn check_if_win(&mut self) -> bool {
    self
      .gameboard
      .coordinates_connected_to_three_in_a_row(&self.gameboard.last_modified_tile)
  }

  pub fn player_symbol_to_game_state(&mut self) -> GameState {
    match self.player_symbol {
      BoardStates::X => GameState::XWon,
      BoardStates::O => GameState::OWon,
      _ => {
        if self.gameboard.tiles_covered == 9 {
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

  while gameconfig.gameboard.tiles_covered < 9 {
    gameconfig.gameboard.print_board();
    println!();

    if gameconfig.player_turn {
      let selected_tile = match parse_player_input() {
        Ok(DEBUG_COORDS) => {
          free_play(&mut gameconfig);
          return Ok(());
        }
        Ok(x) => x,
        Err(_) => continue,
      };

      println!("selected tile = {:?}", selected_tile);

      if gameconfig.gameboard.get_board_state(&selected_tile) == &BoardStates::Empty {
        gameconfig
          .gameboard
          .place_tile(&selected_tile, gameconfig.player_symbol.clone()); // implement AsRef so you don't have to clone here

        gameconfig.gameboard.tiles_covered += 1;
        gameconfig.player_turn = false;
      }

      if gameconfig.check_if_win() {
        gameconfig.gamestate = gameconfig.player_symbol_to_game_state();
        break;
      }
    } else {
      // insert bot code

      //temp stuff until i do add it
      println!(" -- bot turn over -- ");
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

  if player_input.trim().to_lowercase() == "debug" {
    return Ok(DEBUG_COORDS);
  }

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

pub fn free_play(gameconfig: &mut GameConfig) {
  while gameconfig.gameboard.tiles_covered < 9 {
    gameconfig.gameboard.print_board();

    let place_symbol = if gameconfig.player_turn {
      &gameconfig.player_symbol
    } else {
      &gameconfig.bot.bot_symbol
    };

    let selected_tile = match parse_player_input() {
      Ok(coords) => {
        if gameconfig.gameboard.get_board_state(&coords) == &BoardStates::Empty {
          coords
        } else {
          continue;
        }
      }
      Err(_) => continue,
    };

    gameconfig
      .gameboard
      .place_tile(&selected_tile, place_symbol.clone());

    if gameconfig.check_if_win() {
      gameconfig.gameboard.print_board();

      break;
    }

    if gameconfig.player_turn {
      gameconfig.player_turn = false
    } else {
      gameconfig.player_turn = true
    }
  }
}
