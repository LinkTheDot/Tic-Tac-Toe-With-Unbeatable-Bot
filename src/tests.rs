use crate::bot::*;
use crate::coordinate_methods::*;
use crate::gameboard::*;
use crate::gameplay::*;

#[cfg(test)]
mod coordinate_method_tests {
  use super::*;

  #[test]
  fn calculate_opposing_coordinates_logic_works() {
    let origin = (0, 0);
    let adjacent_edge_1 = (0, 1);
    let adjacent_edge_2 = (1, 0);
    let center = (1, 1);

    let opposite_1 = origin.get_opposite_coordinates(&adjacent_edge_1);
    let opposite_2 = origin.get_opposite_coordinates(&adjacent_edge_2);
    let opposite_3 = origin.get_opposite_coordinates(&center);

    assert_eq!(opposite_1, (0, 2));
    assert_eq!(opposite_2, (2, 0));
    assert_eq!(opposite_3, (2, 2));
  }

  #[test]
  fn is_across_and_diagonal_logic_works() {
    let origin = (0, 0);
    let diagonal = (1, 1);
    let horizontal = (0, 1);

    let diagonal_from = diagonal.is_diagonal_from(&origin);
    let horizontal_from = horizontal.is_across_from(&origin);
    let not_diagonal_from = (3, 3).is_diagonal_from(&origin);
    let not_horizontal_from = (3, 3).is_across_from(&origin);

    assert_eq!(diagonal_from, true);
    assert_eq!(horizontal_from, true);
    assert_eq!(not_diagonal_from, false);
    assert_eq!(not_horizontal_from, false);
  }

  #[test]
  fn is_matching_in_a_row_logic_works() {
    let mut game_board = BoardConfig::new();
    let origin_corner = (0, 0);
    let adjacent_for_corner = (0, 1);
    let origin_edge = (1, 0);
    let adjacent_for_edge = (1, 1);

    game_board.tiles[0][0].board_state = BoardStates::X;
    game_board.tiles[0][1].board_state = BoardStates::X;
    game_board.tiles[0][2].board_state = BoardStates::X;

    game_board.tiles[1][0].board_state = BoardStates::O;

    let real_matches = origin_corner.is_matching_in_a_row(&adjacent_for_corner, &game_board);
    let fake_matches = origin_edge.is_matching_in_a_row(&adjacent_for_edge, &game_board);

    assert_eq!(real_matches, true);
    assert_eq!(fake_matches, false);
  }

  #[test]
  fn is_inbetween_logic_works() {
    let game_board = BoardConfig::new();
    let origin = (1, 0);
    let adjacent_match = (1, 1);
  }
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

    game_config.game_board.last_modified_tile = latest_tile_true;
    assert_eq!(game_config.check_if_win(), true);
    game_config.game_board.last_modified_tile = latest_tile_false;
    assert_eq!(game_config.check_if_win(), false);
    game_config.game_board.last_modified_tile = latest_tile_center;
    assert_eq!(game_config.check_if_win(), false);
  }
}

#[cfg(test)]
mod board_tests {
  use super::*;

  #[test]
  pub fn boardconfig_new_works() {
    let board = BoardConfig::new();

    assert_eq!(board.tiles_covered, 0);
  }

  #[test]
  pub fn get_valid_coordinates_around_works() {
    let corner_dummy_coords = vec![(0, 0), (2, 0), (0, 2), (2, 2)];

    let edge_dummy_coords = vec![(1, 0), (0, 1), (1, 2), (2, 1)];

    let center_dummy_coords = (1, 1);
    let mut corner_coords_around_lengths: Vec<usize> = Vec::new();
    let mut edge_coords_around_lengths: Vec<usize> = Vec::new();
    let mut center_coords_around_length: usize;

    for dummy_coords in corner_dummy_coords {
      let coords = get_valid_coordinates_around(&dummy_coords);

      corner_coords_around_lengths.push(coords.len());
    }

    for dummy_coords in edge_dummy_coords {
      let coords = get_valid_coordinates_around(&dummy_coords);

      edge_coords_around_lengths.push(coords.len());
    }

    {
      let coords = get_valid_coordinates_around(&center_dummy_coords);

      center_coords_around_length = coords.len();
    }

    assert_eq!(vec![3, 3, 3, 3], corner_coords_around_lengths);
    assert_eq!(vec![5, 5, 5, 5], edge_coords_around_lengths);
    assert_eq!(8, center_coords_around_length);
  }

  #[test]
  pub fn matching_adjacent_tiles_logic_works() {
    let mut board_config = BoardConfig::new();

    board_config.tiles[0][0].board_state = BoardStates::X;
    board_config.tiles[1][0].board_state = BoardStates::X;

    let adjacent_matched_tiles = board_config.matching_adjacent_tiles(&(0, 0));
    let adjacent_empty_tile = board_config.matching_adjacent_tiles(&(1, 1));
    let empty_vec_because_bugged: Vec<Coordinates> = vec![];

    // checks if it returns real matching symbols
    assert_eq!(vec![(1, 0)], adjacent_matched_tiles);
    // checks if it returns nothing when the tile symbol is Empty
    assert_eq!(empty_vec_because_bugged, adjacent_empty_tile);
  }
}

#[cfg(test)]
mod gameboard_tests {
  use super::*;

  #[test]
  pub fn boardconfig_new_works() {
    let board = BoardConfig::new();

    assert_eq!(board.tiles_covered, 0);
  }

  #[test]
  pub fn get_valid_coordinates_around_works() {
    let corner_dummy_coords = vec![(0, 0), (2, 0), (0, 2), (2, 2)];

    let edge_dummy_coords = vec![(1, 0), (0, 1), (1, 2), (2, 1)];

    let center_dummy_coords = (1, 1);
    let mut corner_coords_around_lengths: Vec<usize> = Vec::new();
    let mut edge_coords_around_lengths: Vec<usize> = Vec::new();
    let mut center_coords_around_length: usize;

    for dummy_coords in corner_dummy_coords {
      let coords = get_valid_coordinates_around(&dummy_coords);

      corner_coords_around_lengths.push(coords.len());
    }

    for dummy_coords in edge_dummy_coords {
      let coords = get_valid_coordinates_around(&dummy_coords);

      edge_coords_around_lengths.push(coords.len());
    }

    {
      let coords = get_valid_coordinates_around(&center_dummy_coords);

      center_coords_around_length = coords.len();
    }

    assert_eq!(vec![3, 3, 3, 3], corner_coords_around_lengths);
    assert_eq!(vec![5, 5, 5, 5], edge_coords_around_lengths);
    assert_eq!(8, center_coords_around_length);
  }

  #[test]
  pub fn matching_adjacent_tiles_logic_works() {
    let mut board_config = BoardConfig::new();

    board_config.tiles[0][0].board_state = BoardStates::X;
    board_config.tiles[1][0].board_state = BoardStates::X;

    let adjacent_matched_tiles = board_config.matching_adjacent_tiles(&(0, 0));
    let adjacent_empty_tile = board_config.matching_adjacent_tiles(&(1, 1));
    let empty_vec_because_bugged: Vec<Coordinates> = vec![];

    // checks if it returns real matching symbols
    assert_eq!(vec![(1, 0)], adjacent_matched_tiles);
    // checks if it returns nothing when the tile symbol is Empty
    assert_eq!(empty_vec_because_bugged, adjacent_empty_tile);
  }
}

#[cfg(test)]
mod bot_tests {
  use super::*;

  #[test]
  pub fn test_bot_new() {
    let new_bot = Bot::new();

    assert_eq!(new_bot.personal_board, BoardConfig::new());
    assert_eq!(new_bot.win_mode, WinChances::High);
  }

  #[test]
  pub fn not_center_checks_logic_works() {
    let mut gameboard = BoardConfig::new();
    let mut bot = Bot::new();

    // where X is player
    // where O is bot

    gameboard.place_tile((1, 1), BoardStates::X);

    bot.path = bot.check_if_center_or_not(&gameboard);
    bot.chosen_placement = bot.initial_check_of_player_center_paths(&gameboard);

    gameboard.place_tile(
      *bot.chosen_placement.as_ref().unwrap_or_else(|err| {
        eprintln!("An error has occurred '{}'", err);
        &(10, 10)
      }),
      BoardStates::O,
    );

    //player places on any of the 3 open corners
    {
      gameboard.place_tile(gameboard.get_random_empty_corner().unwrap(), BoardStates::X);

      bot.chosen_placement = bot.not_center_corner_checks(&gameboard);

      gameboard.place_tile(
        *bot.chosen_placement.as_ref().unwrap_or_else(|err| {
          eprintln!("An error has occurred '{}'", err);
          &(10, 10)
        }),
        BoardStates::O,
      );

      gameboard.print_board();
    }

    //player places on any of the 3 open edges
    {
      gameboard.place_tile(gameboard.get_random_empty_edge().unwrap(), BoardStates::X);

      bot.chosen_placement = bot.not_center_edge_checks(&gameboard);

      gameboard.place_tile(
        *bot.chosen_placement.as_ref().unwrap_or_else(|err| {
          eprintln!("An error has occurred '{}'", err);
          &(10, 10)
        }),
        BoardStates::O,
      );

      gameboard.print_board();
    }
  }
}
