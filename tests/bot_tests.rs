use tictactoe_with_ai::bot::*;
use tictactoe_with_ai::coordinate_methods::Coordinates;
use tictactoe_with_ai::gameboard::*;
use tictactoe_with_ai::gameplay::GameConfig;

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
    fn path_is_unknown() {
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
    #[ignore]
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

      println!(" -- BOT --\n\n{:#?}", &bot);
      println!("\n\n\n\n\n");
      println!(" -- GAMEBOARD --\n\n{:#?}", &gameboard);

      //-|X|-
      //-|O|-
      //O|O|X
      gameboard.place_tile(&(2, 0), BoardStates::X);

      bot.chosen_placement = bot.not_center_corner_checks(&gameboard);

      assert_eq!(bot.chosen_placement, expected_coordinates);
    }
  }
}

#[cfg(test)]
mod block_player_win_logic {
  use super::*;

  #[test]
  fn bot_has_two_in_series() {
    let mut gameconfig = GameConfig::new();

    //X|-|-
    //X|O|-
    //-|-|-
    gameconfig.gameboard.place_tile(&(0, 0), BoardStates::X);
    gameconfig.gameboard.place_tile(&(1, 0), BoardStates::X);
    gameconfig.gameboard.place_tile(&(1, 1), BoardStates::O);

    gameconfig.bot.last_placed_tile = Ok((0, 0));

    gameconfig.bot.chosen_placement = gameconfig.bot.block_player_win(&gameconfig.gameboard);

    assert_eq!(gameconfig.bot.chosen_placement, Ok((2, 0)));
  }

  #[test]
  fn player_has_two_in_series() {
    let mut gameconfig = GameConfig::new();
    let expected_coordinates = Ok((2, 0));

    //X|-|-
    //X|O|-
    //-|-|-
    gameconfig.gameboard.place_tile(&(0, 0), BoardStates::X);
    gameconfig.gameboard.place_tile(&(1, 0), BoardStates::X);
    gameconfig.gameboard.place_tile(&(1, 1), BoardStates::O);

    gameconfig.bot.last_placed_tile = Ok((1, 1));

    gameconfig.bot.chosen_placement = gameconfig.bot.block_player_win(&gameconfig.gameboard);

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

    gameconfig.bot.chosen_placement = gameconfig.bot.block_player_win(&gameconfig.gameboard);

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

    gameconfig.bot.chosen_placement = gameconfig.bot.block_player_win(&gameconfig.gameboard);

    let chosen_coordinate_state = gameconfig
      .gameboard
      .get_board_state(&gameconfig.bot.chosen_placement.unwrap());

    assert_eq!(*chosen_coordinate_state, expected_boardstate);
  }

  #[test]
  fn board_is_full_force_error() {
    let mut gameconfig = GameConfig::new();
    let expected_error = Err("no possible tile to place on".to_string());

    for row in &mut gameconfig.gameboard.tiles {
      for mut tile in row {
        tile.board_state = BoardStates::X;
      }
    }

    gameconfig.gameboard.place_tile(&(2, 2), BoardStates::O);

    gameconfig.bot.last_placed_tile = Ok((0, 0));

    gameconfig.bot.chosen_placement = gameconfig.bot.block_player_win(&gameconfig.gameboard);

    assert_eq!(gameconfig.bot.chosen_placement, expected_error);
  }
}
