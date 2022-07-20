use crate::bot::*;
use crate::coordinate_methods::*;
use crate::gameboard::*;
use std::env::*;
use std::error::Error;
use std::io;

const DEBUG_PLAY: Coordinates = (10, 10);
const BOT_PLAY: Coordinates = (11, 11);

#[derive(PartialEq, Debug)]
pub struct GameConfig {
  pub player_turn: bool,
  pub player_symbol: BoardStates,
  pub gameboard: BoardConfig,
  pub end_gamestate: GameState,
  pub bot: Bot,
}

impl GameConfig {
  pub fn new() -> GameConfig {
    let player_turn = rand::random::<bool>();
    let gameboard = BoardConfig::new();
    let end_gamestate = GameState::Draw;
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
      end_gamestate,
      bot,
    }
  }

  pub fn check_if_win(&mut self) -> bool {
    self
      .gameboard
      .matching_adjacent_tiles(&self.gameboard.last_modified_tile)
      .iter()
      .filter(|coords| {
        self
          .gameboard
          .last_modified_tile
          .is_matching_in_a_row(coords, &self.gameboard)
      })
      .count()
      != 0
  }

  pub fn last_placed_tile_to_game_state(&mut self) -> GameState {
    match self
      .gameboard
      .get_board_state(&self.gameboard.last_modified_tile)
    {
      BoardStates::X => GameState::XWon,
      BoardStates::O => GameState::OWon,
      _ => GameState::Draw,
    }
  }
}

#[derive(PartialEq, Clone, Debug)]
pub enum GameState {
  XWon,
  OWon,
  Draw,
}

pub fn run_gameplay() -> Result<(), Box<dyn Error>> {
  let mut gameconfig = GameConfig::new();

  while gameconfig.gameboard.tiles_covered < 9 {
    gameconfig.gameboard.print_board();
    println!();

    if gameconfig.player_turn {
      let selected_tile = match parse_player_input() {
        Ok(x) => x,
        Err(_) => continue,
      };

      if gameconfig.gameboard.get_board_state(&selected_tile) == &BoardStates::Empty {
        gameconfig
          .gameboard
          .place_tile(&selected_tile, &gameconfig.player_symbol);

        gameconfig.gameboard.tiles_covered += 1;
        gameconfig.player_turn = false;
      }
    } else {
      gameconfig.bot.choose_coordinates(&gameconfig.gameboard);

      gameconfig.gameboard.place_tile(
        gameconfig.bot.most_recent_chosen_coords.as_ref().unwrap(),
        &gameconfig.bot.bot_symbol,
      );

      println!(" -- bot turn over --\n");

      gameconfig.gameboard.tiles_covered += 1;
      gameconfig.player_turn = true;
    }

    if gameconfig.check_if_win() {
      gameconfig.end_gamestate = gameconfig.last_placed_tile_to_game_state();
      break;
    }
  }

  println!("{:?}", gameconfig.end_gamestate);
  gameconfig.gameboard.print_board();

  Ok(())
}

pub fn parse_player_input() -> Result<Coordinates, Box<dyn Error>> {
  println!("Select a tile 1-9 or a1, b2, c3, etc.");

  let mut player_input = String::new();
  io::stdin().read_line(&mut player_input).unwrap();

  if player_input.trim().len() == 1 {
    return index_parsing(player_input);
  } else if player_input.trim().len() == 2 {
    return row_column_parsing(player_input);
  }

  Err(Box::from("incorrect input"))
}

fn row_column_parsing(player_input: String) -> Result<Coordinates, Box<dyn Error>> {
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

  Err(Box::from("incorrect input"))
}

fn index_parsing(player_input: String) -> Result<Coordinates, Box<dyn Error>> {
  if let Ok(num) = player_input.trim().parse::<usize>() {
    if num != 0 {
      return Ok(((num - 1) / GRID_SIZE, (num - 1) % GRID_SIZE));
    }
  }

  Err(Box::from("incorrect input"))
}

pub fn free_play() -> Result<(), Box<dyn Error>> {
  let mut gameconfig = GameConfig::new();

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
      .place_tile(&selected_tile, place_symbol);

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

  println!("{:?}", gameconfig.end_gamestate);
  gameconfig.gameboard.print_board();

  Ok(())
}

pub fn bot_play() -> Result<(), Box<dyn Error>> {
  use std::thread;
  use std::time::Duration;

  let mut gameconfig = GameConfig::new();
  let mut second_bot = Bot::new();

  for row in &mut gameconfig.gameboard.tiles {
    for mut tile in row {
      tile.board_state = BoardStates::Empty;
    }
  }

  second_bot.bot_symbol = gameconfig.player_symbol.clone();

  while gameconfig.gameboard.tiles_covered < 9 {
    gameconfig.gameboard.print_board();

    if gameconfig.player_turn {
      gameconfig.bot.choose_coordinates(&gameconfig.gameboard);

      gameconfig.gameboard.place_tile(
        gameconfig.bot.most_recent_chosen_coords.as_ref().unwrap(),
        &gameconfig.bot.bot_symbol,
      );

      gameconfig.player_turn = false;

      println!(" -- bot 1 turn over --\n");

      thread::sleep(Duration::from_millis(200));
    } else {
      second_bot.choose_coordinates(&gameconfig.gameboard);

      gameconfig.gameboard.place_tile(
        second_bot.most_recent_chosen_coords.as_ref().unwrap(),
        &second_bot.bot_symbol,
      );

      gameconfig.player_turn = true;

      println!(" -- bot 2 turn over --\n");

      thread::sleep(Duration::from_millis(200));
    }

    if gameconfig.check_if_win() {
      gameconfig.end_gamestate = gameconfig.last_placed_tile_to_game_state();
      break;
    }
  }

  println!("{:?}", gameconfig.end_gamestate);
  gameconfig.gameboard.print_board();

  Ok(())
}

pub fn run_args(user_input: Vec<String>) -> Result<(), Box<dyn Error>> {
  match user_input[1].to_lowercase().trim() {
    "bot_play" => return bot_play(),
    "free_play" => return free_play(),
    _ => return Ok(()),
  }
}
