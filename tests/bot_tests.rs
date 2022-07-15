use tictactoe_with_ai::bot::*;
use tictactoe_with_ai::coordinate_methods::Coordinates;
use tictactoe_with_ai::gameboard::*;
use tictactoe_with_ai::gameplay::GameConfig;

// == MAKE A GLOBAL VARIABLE FOR PAYER AND BOT BOARDSTATES ==

#[cfg(test)]
mod not_center_logic {
  use super::*;

  #[cfg(test)]
  mod edge_check {
    use super::*;

    #[test]
    fn player_places_edge_near_corner() {
      let mut bot = Bot::new();
      let mut gameboard = BoardConfig::new();
      let expected_coordinates: Result<Coordinates, String> = Ok((0, 1));

      // where O is player
      // where X is bot

      //-|-|-
      //-|O|-
      //-|O|X
      gameboard.place_tile(&(1, 1), BoardStates::O);
      gameboard.place_tile(&(2, 2), BoardStates::X);
      gameboard.place_tile(&(2, 1), BoardStates::O);

      bot.last_placed_tile = Ok((2, 2));
      bot.chosen_placement = bot.not_center_edge_checks(&gameboard);

      assert_eq!(bot.chosen_placement, expected_coordinates);
    }

    #[test]
    fn player_places_edge_away_from_corner() {
      let mut bot = Bot::new();
      let mut gameboard = BoardConfig::new();
      let expected_coordinates: Result<Coordinates, String> = Ok((2, 1));

      // where O is player
      // where X is bot

      //-|O|-
      //-|O|-
      //-|-|X
      gameboard.place_tile(&(1, 1), BoardStates::O);
      gameboard.place_tile(&(2, 2), BoardStates::X);
      gameboard.place_tile(&(0, 1), BoardStates::O);

      bot.last_placed_tile = Ok((2, 2));
      bot.chosen_placement = bot.not_center_edge_checks(&gameboard);

      assert_eq!(bot.chosen_placement, expected_coordinates);
    }
  }

  #[cfg(test)]
  mod corner_tests {
    use super::*;

    #[test]
    fn initial_corner_check() {
      let mut bot = Bot::new();
      let mut gameboard = BoardConfig::new();
      let expected_coordinates = Ok((2, 0));

      // where O is player
      // where X is bot

      //-|-|O
      //-|O|-
      //-|-|X
      gameboard.place_tile(&(1, 1), BoardStates::O);
      gameboard.place_tile(&(2, 2), BoardStates::X);
      gameboard.place_tile(&(0, 2), BoardStates::O);

      bot.last_placed_tile = Ok((2, 2));

      bot.path = CurrentPath::NotCenter(PlayerCenterPaths::Unknown);

      bot.chosen_placement = bot.not_center_corner_checks(&gameboard);

      assert_eq!(bot.chosen_placement, expected_coordinates);
    }

    #[test]
    fn player_placed_edge_near() {
      let mut bot = Bot::new();
      let mut gameboard = BoardConfig::new();
      let expected_coordinates: Result<Coordinates, String> = Ok((0, 2));

      // where O is player
      // where X is bot

      // goes in order of placement
      //-|-|-
      //-|O|-
      //-|O|X
      gameboard.place_tile(&(1, 1), BoardStates::O);
      gameboard.place_tile(&(2, 2), BoardStates::X);
      gameboard.place_tile(&(2, 1), BoardStates::O);

      bot.last_placed_tile = Ok((2, 2));

      bot.chosen_placement = bot.not_center_edge_checks(&gameboard);

      gameboard.place_tile(bot.chosen_placement.as_ref().unwrap(), BoardStates::X);

      //-|X|-
      //-|O|-
      //O|O|X
      gameboard.place_tile(&(2, 0), BoardStates::O);

      bot.chosen_placement = bot.not_center_corner_checks(&gameboard);

      assert_eq!(bot.chosen_placement, expected_coordinates);
    }
  }
}

#[cfg(test)]
mod center_checks {
  #![allow(unused)]
  use super::*;

  // might need a few more tests for corner?

  #[cfg(test)]
  mod edge_checks {
    use super::*;

    #[test]
    fn initial_edge_check() {
      let mut gameboard = BoardConfig::new();
      let mut bot = Bot::new();
      let expected_chosen_placement_one = Ok((2, 2));
      // put another one if i end up making it random

      //Where X is bot and O is player

      //-|-|-
      //-|X|-
      //-|O|-
      gameboard.place_tile(&(1, 1), BoardStates::X);
      gameboard.place_tile(&(2, 1), BoardStates::O);

      bot.last_placed_tile = Ok((1, 1));
      bot.path = CurrentPath::Center(BotCenterPaths::Unknown);

      bot.chosen_placement = bot.center_edge_checks(&gameboard);

      assert_eq!(bot.chosen_placement, expected_chosen_placement_one);
    }

    #[test]
    fn player_placed_edge_then_edge_again() {
      let mut gameboard = BoardConfig::new();
      let mut bot = Bot::new();
      let expected_chosen_placement = Ok((0, 0));

      bot.path = CurrentPath::Center(BotCenterPaths::Unknown);

      // Where X is bot and O is player

      //-|-|-
      //-|X|-
      //-|O|-
      gameboard.place_tile(&(1, 1), BoardStates::X);
      gameboard.place_tile(&(2, 1), BoardStates::O);

      bot.last_placed_tile = Ok((1, 1));

      bot.chosen_placement = bot.center_edge_checks(&gameboard);
      bot.last_placed_tile = bot.chosen_placement.clone();

      //-|-|-
      //-|X|O
      //-|O|X
      gameboard.place_tile(bot.chosen_placement.as_ref().unwrap(), BoardStates::X);
      gameboard.place_tile(&(1, 2), BoardStates::O);

      bot.chosen_placement = bot.center_edge_checks(&gameboard);

      assert_eq!(bot.chosen_placement, expected_chosen_placement)
    }
  }

  #[cfg(test)]
  mod corner_checks {
    use super::*;

    #[test]
    fn initial_corner_check() {
      let mut gameboard = BoardConfig::new();
      let mut bot = Bot::new();
      let expected_chosen_placement = Ok((0, 0));

      bot.path = CurrentPath::Center(BotCenterPaths::Unknown);

      //-|-|-
      //-|X|-
      //-|-|O
      gameboard.place_tile(&(1, 1), BoardStates::X);
      gameboard.place_tile(&(2, 2), BoardStates::O);

      bot.chosen_placement = bot.center_corner_checks(&gameboard);

      assert_eq!(bot.chosen_placement, expected_chosen_placement);
    }

    #[test]
    fn placed_corner_then_corner() {
      let mut gameboard = BoardConfig::new();
      let mut bot = Bot::new();
      let expected_chosen_placement = Ok((2, 1));

      bot.path = CurrentPath::Center(BotCenterPaths::Unknown);

      //X|-|-
      //-|X|-
      //O|-|O
      gameboard.place_tile(&(1, 1), BoardStates::X);
      gameboard.place_tile(&(2, 2), BoardStates::O);

      bot.chosen_placement = bot.center_corner_checks(&gameboard);
      bot.last_placed_tile = bot.chosen_placement.clone();

      gameboard.place_tile(bot.chosen_placement.as_ref().unwrap(), BoardStates::X);
      gameboard.place_tile(&(2, 0), BoardStates::O);

      bot.chosen_placement = bot.center_corner_checks(&gameboard);

      assert_eq!(bot.chosen_placement, expected_chosen_placement)
    }
  }
}

#[cfg(test)]
mod block_player_win_logic {
  use super::*;

  #[test]
  fn bot_has_two_in_series() {
    let mut gameconfig = GameConfig::new();
    let expected_coordinates = Ok((2, 0));

    //X|-|-
    //X|O|-
    //-|-|-
    gameconfig.gameboard.place_tile(&(0, 0), BoardStates::X);
    gameconfig.gameboard.place_tile(&(1, 0), BoardStates::X);
    gameconfig.gameboard.place_tile(&(1, 1), BoardStates::O);

    gameconfig.bot.last_placed_tile = Ok((0, 0));
    gameconfig.bot.chosen_placement = gameconfig.bot.auto_play(&gameconfig.gameboard);

    assert_eq!(gameconfig.bot.chosen_placement, expected_coordinates);
  }

  #[test]
  fn player_has_two_in_series() {
    let mut gameconfig = GameConfig::new();
    let expected_coordinates = Ok((2, 0));

    //O|-|-
    //O|X|-
    //-|-|-
    gameconfig.gameboard.place_tile(&(1, 0), BoardStates::O);
    gameconfig.gameboard.place_tile(&(1, 1), BoardStates::X);
    gameconfig.gameboard.place_tile(&(0, 0), BoardStates::O);

    gameconfig.bot.last_placed_tile = Ok((1, 1));
    gameconfig.bot.chosen_placement = gameconfig.bot.auto_play(&gameconfig.gameboard);

    assert_eq!(gameconfig.bot.chosen_placement, expected_coordinates);
  }

  #[test]
  fn no_possible_wins() {
    let mut gameconfig = GameConfig::new();
    let expected_boardstate = BoardStates::Empty;

    //X|-|-
    //-|O|X
    //-|-|-
    gameconfig.gameboard.place_tile(&(0, 0), BoardStates::X);
    gameconfig.gameboard.place_tile(&(2, 1), BoardStates::X);
    gameconfig.gameboard.place_tile(&(1, 1), BoardStates::O);

    gameconfig.bot.last_placed_tile = Ok((0, 0));

    gameconfig.bot.chosen_placement = gameconfig.bot.auto_play(&gameconfig.gameboard);

    let chosen_coordinate_state = gameconfig
      .gameboard
      .get_board_state(&gameconfig.bot.chosen_placement.unwrap());

    assert_eq!(*chosen_coordinate_state, expected_boardstate);
  }

  #[test]
  fn potential_overflow() {
    let mut gameconfig = GameConfig::new();
    let expected_boardstate = BoardStates::Empty;

    //O|X|-
    //X|-|-
    //-|X|-
    gameconfig.gameboard.place_tile(&(1, 0), BoardStates::X);
    gameconfig.gameboard.place_tile(&(0, 1), BoardStates::X);
    gameconfig.gameboard.place_tile(&(2, 1), BoardStates::X);
    gameconfig.gameboard.place_tile(&(0, 0), BoardStates::O);

    gameconfig.bot.last_placed_tile = Ok((1, 0));

    gameconfig.bot.chosen_placement = gameconfig.bot.auto_play(&gameconfig.gameboard);

    let chosen_coordinate_state = gameconfig
      .gameboard
      .get_board_state(&gameconfig.bot.chosen_placement.unwrap());

    assert_eq!(*chosen_coordinate_state, expected_boardstate);
  }

  #[test]
  fn board_is_full_force_error() {
    let mut gameconfig = GameConfig::new();
    let expected_error = Err("No possible tile to place on".to_string());

    for row in &mut gameconfig.gameboard.tiles {
      for mut tile in row {
        tile.board_state = BoardStates::X;
      }
    }

    gameconfig.gameboard.place_tile(&(2, 2), BoardStates::O);

    gameconfig.bot.last_placed_tile = Ok((0, 0));

    gameconfig.bot.chosen_placement = gameconfig.bot.auto_play(&gameconfig.gameboard);

    assert_eq!(gameconfig.bot.chosen_placement, expected_error);
  }
}
