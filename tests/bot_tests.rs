use tictactoe_with_ai::bot::*;
use tictactoe_with_ai::coordinate_methods::Coordinates;
use tictactoe_with_ai::gameboard::*;

const BOT_BOARD_SYMBOL: BoardStates = BoardStates::X;
const PLAYER_BOARD_SYMBOL: BoardStates = BoardStates::O;

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
      let expected_coordinates = Ok((0, 1));

      //-|-|-
      //-|O|-
      //-|O|X
      gameboard.place_tile(&(1, 1), &PLAYER_BOARD_SYMBOL);
      bot.most_recent_chosen_coords = Ok((2, 2));
      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );
      gameboard.place_tile(&(2, 1), &PLAYER_BOARD_SYMBOL);

      bot.most_recent_chosen_coords = bot.not_center_edge_checks(&gameboard);

      assert_eq!(bot.most_recent_chosen_coords, expected_coordinates);
    }

    #[test]
    fn player_places_edge_away_from_corner() {
      let mut bot = Bot::new();
      let mut gameboard = BoardConfig::new();
      let expected_coordinates: Result<Coordinates, String> = Ok((2, 1));

      //-|O|-
      //-|O|-
      //-|-|X
      gameboard.place_tile(&(1, 1), &PLAYER_BOARD_SYMBOL);
      bot.most_recent_chosen_coords = Ok((2, 2));
      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );
      gameboard.place_tile(&(0, 1), &PLAYER_BOARD_SYMBOL);

      bot.most_recent_chosen_coords = bot.not_center_edge_checks(&gameboard);

      assert_eq!(bot.most_recent_chosen_coords, expected_coordinates);
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

      //-|-|O
      //-|O|-
      //-|-|X
      gameboard.place_tile(&(1, 1), &PLAYER_BOARD_SYMBOL);
      bot.most_recent_chosen_coords = Ok((2, 2));
      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );
      gameboard.place_tile(&(0, 2), &PLAYER_BOARD_SYMBOL);

      bot.path = CurrentPath::NotCenter(PlayerCenterPaths::Unknown);
      bot.most_recent_chosen_coords = bot.not_center_corner_checks(&gameboard);

      assert_eq!(bot.most_recent_chosen_coords, expected_coordinates);
    }

    #[test]
    fn player_placed_edge_near() {
      let mut bot = Bot::new();
      let mut gameboard = BoardConfig::new();
      let expected_coordinates: Result<Coordinates, String> = Ok((0, 2));

      //-|-|-
      //-|O|-
      //-|O|X
      gameboard.place_tile(&(1, 1), &PLAYER_BOARD_SYMBOL);
      bot.most_recent_chosen_coords = Ok((2, 2));
      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );
      gameboard.place_tile(&(2, 1), &PLAYER_BOARD_SYMBOL);

      bot.most_recent_chosen_coords = bot.not_center_edge_checks(&gameboard);

      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );

      //-|X|-
      //-|O|-
      //O|O|X
      gameboard.place_tile(&(2, 0), &PLAYER_BOARD_SYMBOL);

      bot.most_recent_chosen_coords = bot.not_center_corner_checks(&gameboard);

      assert_eq!(bot.most_recent_chosen_coords, expected_coordinates);
    }
  }
}

#[cfg(test)]
mod center_checks {
  #![allow(unused)]
  use super::*;

  #[cfg(test)]
  mod edge_checks {
    use super::*;

    #[test]
    fn initial_edge_check() {
      let mut gameboard = BoardConfig::new();
      let mut bot = Bot::new();
      let expected_chosen_placement_one = Ok((2, 2));
      // put another one if i end up making it random

      //-|-|-
      //-|X|-
      //-|O|-
      bot.most_recent_chosen_coords = Ok((1, 1));
      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );
      gameboard.place_tile(&(2, 1), &PLAYER_BOARD_SYMBOL);

      bot.path = CurrentPath::Center(BotCenterPaths::Unknown);

      bot.most_recent_chosen_coords = bot.center_edge_checks(&gameboard);

      assert_eq!(bot.most_recent_chosen_coords, expected_chosen_placement_one);
    }

    #[test]
    fn player_placed_edge_then_edge_again() {
      let mut gameboard = BoardConfig::new();
      let mut bot = Bot::new();
      let expected_chosen_placement = Ok((0, 0));

      bot.path = CurrentPath::Center(BotCenterPaths::Unknown);

      //-|-|-
      //-|X|-
      //-|O|-
      bot.most_recent_chosen_coords = Ok((1, 1));
      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );
      gameboard.place_tile(&(2, 1), &PLAYER_BOARD_SYMBOL);

      bot.most_recent_chosen_coords = bot.center_edge_checks(&gameboard);

      //-|-|-
      //-|X|O
      //-|O|X
      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );
      gameboard.place_tile(&(1, 2), &PLAYER_BOARD_SYMBOL);

      bot.most_recent_chosen_coords = bot.center_edge_checks(&gameboard);

      assert_eq!(bot.most_recent_chosen_coords, expected_chosen_placement)
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
      gameboard.place_tile(&(1, 1), &BOT_BOARD_SYMBOL);
      gameboard.place_tile(&(2, 2), &PLAYER_BOARD_SYMBOL);

      bot.most_recent_chosen_coords = bot.center_corner_checks(&gameboard);

      assert_eq!(bot.most_recent_chosen_coords, expected_chosen_placement);
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
      bot.most_recent_chosen_coords = Ok((1, 1));
      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );
      gameboard.place_tile(&(2, 2), &PLAYER_BOARD_SYMBOL);

      bot.most_recent_chosen_coords = bot.center_corner_checks(&gameboard);

      gameboard.place_tile(
        bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );
      gameboard.place_tile(&(2, 0), &PLAYER_BOARD_SYMBOL);

      bot.most_recent_chosen_coords = bot.center_corner_checks(&gameboard);

      assert_eq!(bot.most_recent_chosen_coords, expected_chosen_placement)
    }
  }
}

#[cfg(test)]
mod auto_play_logic {
  use super::*;

  #[test]
  fn bot_has_two_in_series() {
    let mut gameboard = BoardConfig::new();
    let mut bot = Bot::new();
    let expected_coordinates = Ok((2, 0));

    //X|-|-
    //X|O|-
    //-|-|-
    gameboard.place_tile(&(1, 0), &BOT_BOARD_SYMBOL);
    bot.most_recent_chosen_coords = Ok((0, 0));
    gameboard.place_tile(
      bot.most_recent_chosen_coords.as_ref().unwrap(),
      &BOT_BOARD_SYMBOL,
    );

    gameboard.place_tile(&(1, 1), &PLAYER_BOARD_SYMBOL);

    bot.most_recent_chosen_coords = bot.auto_play(&gameboard);

    assert_eq!(bot.most_recent_chosen_coords, expected_coordinates);
  }

  #[test]
  fn player_has_two_in_series() {
    let mut gameboard = BoardConfig::new();
    let mut bot = Bot::new();
    let expected_coordinates = Ok((2, 0));

    //O|-|-
    //O|X|-
    //-|-|-
    gameboard.place_tile(&(1, 0), &PLAYER_BOARD_SYMBOL);
    bot.most_recent_chosen_coords = Ok((1, 1));
    gameboard.place_tile(
      bot.most_recent_chosen_coords.as_ref().unwrap(),
      &BOT_BOARD_SYMBOL,
    );

    gameboard.place_tile(&(0, 0), &PLAYER_BOARD_SYMBOL);

    bot.most_recent_chosen_coords = bot.auto_play(&gameboard);

    assert_eq!(bot.most_recent_chosen_coords, expected_coordinates);
  }

  #[test]
  fn no_possible_wins() {
    let mut gameboard = BoardConfig::new();
    let mut bot = Bot::new();
    let expected_boardstate = BoardStates::Empty;

    //X|-|-
    //-|O|X
    //-|-|-
    gameboard.place_tile(&(2, 1), &BOT_BOARD_SYMBOL);
    bot.most_recent_chosen_coords = Ok((0, 0));
    gameboard.place_tile(
      bot.most_recent_chosen_coords.as_ref().unwrap(),
      &BOT_BOARD_SYMBOL,
    );

    gameboard.place_tile(&(1, 1), &PLAYER_BOARD_SYMBOL);

    bot.most_recent_chosen_coords = bot.auto_play(&gameboard);

    let chosen_coordinate_state =
      gameboard.get_board_state(&bot.most_recent_chosen_coords.unwrap());

    assert_eq!(*chosen_coordinate_state, expected_boardstate);
  }

  #[test]
  fn potential_overflow() {
    let mut gameboard = BoardConfig::new();
    let mut bot = Bot::new();
    let expected_boardstate = BoardStates::Empty;

    //O|X|-
    //X|-|-
    //-|X|-
    gameboard.place_tile(&(0, 1), &BOT_BOARD_SYMBOL);
    gameboard.place_tile(&(2, 1), &BOT_BOARD_SYMBOL);
    bot.most_recent_chosen_coords = Ok((1, 0));
    gameboard.place_tile(
      bot.most_recent_chosen_coords.as_ref().unwrap(),
      &BOT_BOARD_SYMBOL,
    );

    gameboard.place_tile(&(0, 0), &PLAYER_BOARD_SYMBOL);

    bot.most_recent_chosen_coords = bot.auto_play(&gameboard);

    let chosen_coordinate_state =
      gameboard.get_board_state(&bot.most_recent_chosen_coords.unwrap());

    assert_eq!(*chosen_coordinate_state, expected_boardstate);
  }

  #[test]
  fn board_is_full_force_error() {
    let mut gameboard = BoardConfig::new();
    let mut bot = Bot::new();
    let expected_error = Err("No possible tile to place on".to_string());

    for row in &mut gameboard.tiles {
      for mut tile in row {
        tile.board_state = BOT_BOARD_SYMBOL;
      }
    }

    gameboard.place_tile(&(2, 2), &PLAYER_BOARD_SYMBOL);

    bot.most_recent_chosen_coords = Ok((0, 0));

    bot.most_recent_chosen_coords = bot.auto_play(&gameboard);

    assert_eq!(bot.most_recent_chosen_coords, expected_error);
  }
}

#[cfg(test)]
mod choose_coordinates_logic {
  #![allow(unused)]
  use super::*;

  #[cfg(test)]
  mod first_two_moves {
    use super::*;

    #[test]
    fn bot_is_first() {
      let mut bot = Bot::new();
      let mut gameboard = BoardConfig::new();
      let expected_chosen_placement = Ok((1, 1));

      bot.choose_coordinates(&gameboard);

      assert_eq!(expected_chosen_placement, bot.most_recent_chosen_coords);
    }

    #[test]
    fn bot_is_second() {
      let mut bot = Bot::new();
      let mut gameboard = BoardConfig::new();
      let player_placement = (1, 1);
      let expected_board_position = BoardPositions::Corner;

      gameboard.place_tile(&player_placement, &PLAYER_BOARD_SYMBOL);

      bot.choose_coordinates(&gameboard);

      let bot_chosen_position =
        gameboard.get_board_position(bot.most_recent_chosen_coords.as_ref().unwrap());

      assert_eq!(bot_chosen_position, &expected_board_position);
    }

    #[test]
    fn bot_is_second_center_is_open() {
      let mut bot = Bot::new();
      let mut gameboard = BoardConfig::new();
      let player_placement = (0, 0);
      let expected_chosen_placement = Ok((1, 1));

      gameboard.place_tile(&player_placement, &PLAYER_BOARD_SYMBOL);

      bot.choose_coordinates(&gameboard);

      assert_eq!(expected_chosen_placement, bot.most_recent_chosen_coords);
    }
  }
}

#[cfg(test)]
mod known_bugs {
  use super::*;
  use tictactoe_with_ai::coordinate_methods::*;
  use tictactoe_with_ai::gameplay::GameConfig;

  #[test]
  fn bot_possible_lose() {
    let first_move = (1, 1);
    let mut bot_edge_placement_count = 0;
    let expected_bot_edge_placements = 0;

    // == overall goal ==
    //
    //       -|-|O
    //       X|O|-
    //       X|-|-
    //

    for _x in 0..25 {
      let mut gameconfig = GameConfig::new()
        .unwrap_or_else(|error| panic!("An error has occured while grabbing config: '{error}'"));

      gameconfig.player_symbol = PLAYER_BOARD_SYMBOL;
      gameconfig.bot.bot_symbol = BOT_BOARD_SYMBOL;

      // -|-|-
      // -|O|-
      // -|-|-
      gameconfig
        .gameboard
        .place_tile(&first_move, &PLAYER_BOARD_SYMBOL);

      // Some corner, it's random
      // -|-|-
      // -|O|-
      // X|-|-
      gameconfig.bot.choose_coordinates(&gameconfig.gameboard);
      gameconfig.gameboard.place_tile(
        &gameconfig.bot.most_recent_chosen_coords.as_ref().unwrap(),
        &BOT_BOARD_SYMBOL,
      );

      // Opposite of the bot's corner
      // -|-|O
      // -|O|-
      // X|-|-
      let second_move = gameconfig
        .gameboard
        .last_modified_tile
        .unwrap()
        .get_opposite_coordinates(&first_move);

      gameconfig
        .gameboard
        .place_tile(&second_move, &PLAYER_BOARD_SYMBOL);

      gameconfig.bot.choose_coordinates(&gameconfig.gameboard);

      let position_of_bot_placement = gameconfig
        .gameboard
        .get_board_position(gameconfig.bot.most_recent_chosen_coords.as_ref().unwrap());

      if *position_of_bot_placement == BoardPositions::Edge {
        bot_edge_placement_count += 1;
      }
    }

    assert_eq!(bot_edge_placement_count, expected_bot_edge_placements);
  }
}
