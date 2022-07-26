use crate::bot::*;
use crate::coordinate_methods::*;
use crate::gameboard::*;
use std::error::Error;
use std::io;

#[derive(PartialEq, Debug)]
pub struct GameConfig {
  pub player_turn: bool,
  pub player_symbol: BoardStates,
  pub gameboard: BoardConfig,
  pub end_gamestate: GameState,
  pub bot: Bot,
}

impl GameConfig {
  pub fn new() -> Result<GameConfig, Box<dyn Error>> {
    let player_turn = rand::random::<bool>();
    let mut bot = Bot::new();

    let player_symbol = if player_turn {
      bot.bot_symbol = BoardStates::O;
      BoardStates::X
    } else {
      bot.bot_symbol = BoardStates::X;
      BoardStates::O
    };

    Ok(GameConfig {
      player_turn,
      player_symbol,
      gameboard: BoardConfig::new(),
      end_gamestate: GameState::Draw,
      bot,
    })
  }

  pub fn check_if_win(&mut self) -> bool {
    self
      .gameboard
      .matching_adjacent_tiles(&self.gameboard.last_modified_tile.unwrap())
      .iter()
      .filter(|coords| {
        self
          .gameboard
          .last_modified_tile
          .unwrap()
          .is_matching_in_a_row(coords, &self.gameboard)
      })
      .count()
      != 0
  }
}

#[derive(PartialEq, Clone, Debug)]
pub enum GameState {
  XWon,
  OWon,
  Draw,
}

pub fn run_gameplay(gameconfig: &mut GameConfig) -> Result<(), Box<dyn Error>> {
  println!("\n\n -- run the program with 'bot_play' or 'free_play' for other modes -- \n\n");

  while gameconfig.gameboard.tiles_covered < 9 {
    println!();

    if gameconfig.player_turn {
      player_turn(gameconfig);

      gameconfig.player_turn = false;
    } else {
      bot_turn(&mut gameconfig.bot, &mut gameconfig.gameboard);

      gameconfig.player_turn = true;
    }

    if gameconfig.check_if_win() {
      gameconfig.end_gamestate = gameconfig.gameboard.last_placed_tile_to_game_state();

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

pub fn free_play(gameconfig: &mut GameConfig) -> Result<(), Box<dyn Error>> {
  let player_one_symbol = gameconfig.player_symbol;
  let player_two_symbol = gameconfig.bot.bot_symbol;

  while gameconfig.gameboard.tiles_covered < 9 {
    println!();

    if gameconfig.player_turn {
      gameconfig.player_symbol = player_one_symbol;
    } else {
      gameconfig.player_symbol = player_two_symbol;
    }

    player_turn(gameconfig);

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

pub fn bot_play(gameconfig: &mut GameConfig) -> Result<(), Box<dyn Error>> {
  use std::thread;
  use std::time::Duration;

  let mut second_bot = Bot::new();

  second_bot.bot_symbol = gameconfig.player_symbol;

  while gameconfig.gameboard.tiles_covered < 9 {
    println!();

    if gameconfig.player_turn {
      bot_turn(&mut gameconfig.bot, &mut gameconfig.gameboard);

      gameconfig.player_turn = false;

      thread::sleep(Duration::from_millis(500));
    } else {
      bot_turn(&mut second_bot, &mut gameconfig.gameboard);

      gameconfig.player_turn = true;

      thread::sleep(Duration::from_millis(500));
    }

    if gameconfig.check_if_win() {
      gameconfig.end_gamestate = gameconfig.gameboard.last_placed_tile_to_game_state();
      break;
    }
  }

  println!("{:?}", gameconfig.end_gamestate);
  gameconfig.gameboard.print_board();

  Ok(())
}

fn player_turn(gameconfig: &mut GameConfig) {
  loop {
    gameconfig.gameboard.print_board();

    let selected_tile = match parse_player_input() {
      Ok(x) => x,
      Err(error) => {
        println!("{error}");

        continue;
      }
    };

    if gameconfig.gameboard.get_board_state(&selected_tile) == &BoardStates::Empty {
      gameconfig
        .gameboard
        .place_tile(&selected_tile, &gameconfig.player_symbol);

      gameconfig.gameboard.tiles_covered += 1;

      break;
    }
  }
}

fn bot_turn(bot: &mut Bot, gameboard: &mut BoardConfig) {
  gameboard.print_board();

  bot.choose_coordinates(gameboard);

  gameboard.place_tile(
    bot.most_recent_chosen_coords.as_ref().unwrap(),
    &bot.bot_symbol,
  );

  println!(" -- bot turn over --\n");

  gameboard.tiles_covered += 1;
}

pub fn check_args_for_gamemodes(
  user_input: String,
  gameconfig: &mut GameConfig,
) -> Result<(), Box<dyn Error>> {
  match user_input.to_lowercase().trim() {
    "bot_play" => bot_play(gameconfig),
    "free_play" => free_play(gameconfig),
    _ => Ok(()),
  }
}
