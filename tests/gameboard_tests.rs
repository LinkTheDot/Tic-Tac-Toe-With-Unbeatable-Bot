use tictactoe_with_ai::coordinate_methods::*;
use tictactoe_with_ai::gameboard::*;

#[test]
fn boardconfig_new_works() {
  let board = BoardConfig::new();

  assert_eq!(board.tiles_covered, 0);
}

#[cfg(test)]
mod get_valid_coordinates_around_logic {
  use super::*;

  #[test]
  fn from_edge_coords() {
    let edge_coords = vec![(1, 0), (0, 1), (1, 2), (2, 1)];
    let expected_edge_return_counts = vec![5, 5, 5, 5];

    let amount_of_coords_around_edges = edge_coords
      .into_iter()
      .map(|coords| get_valid_coordinates_around(&coords).len())
      .collect::<Vec<usize>>();

    assert_eq!(expected_edge_return_counts, amount_of_coords_around_edges);
  }

  #[test]
  fn from_corner_coords() {
    let corner_coords = vec![(0, 0), (2, 0), (0, 2), (2, 2)];
    let expected_corner_return_counts = vec![3, 3, 3, 3];

    let amount_of_coords_around_corners = corner_coords
      .into_iter()
      .map(|coords| get_valid_coordinates_around(&coords).len())
      .collect::<Vec<usize>>();

    assert_eq!(
      expected_corner_return_counts,
      amount_of_coords_around_corners
    );
  }

  #[test]
  fn from_center_coords() {
    let center_coords = (1, 1);
    let expected_center_return_counts = 8;

    let amount_of_coords_around_center = get_valid_coordinates_around(&center_coords).len();
    assert_eq!(
      expected_center_return_counts,
      amount_of_coords_around_center
    );
  }
}

#[test]
fn matching_adjacent_tiles_logic_works() {
  let mut boardconfig = BoardConfig::new();
  let expected_adjacent_match = vec![(1, 0)];
  let expected_empty_value: Vec<Coordinates> = vec![];
  let check_around_x_value = (0, 0);
  let check_around_empty_value = (1, 1);

  //X|-|-
  //X|-|-
  //-|-|-
  boardconfig.place_tile(&(0, 0), BoardStates::X);
  boardconfig.place_tile(&(1, 0), BoardStates::X);

  let adjacent_matched_tiles = boardconfig.matching_adjacent_tiles(&check_around_x_value);
  let adjacent_empty_tile = boardconfig.matching_adjacent_tiles(&check_around_empty_value);

  assert_eq!(expected_adjacent_match, adjacent_matched_tiles);
  assert_eq!(expected_empty_value, adjacent_empty_tile);
}

#[cfg(test)]
mod two_in_series {
  use super::*;

  #[test]
  fn corner_over_edges_with_fake_over() {
    let mut gameboard = BoardConfig::new();
    let expected_coordinates = Some((0, 2));

    // X|X|-
    // X|-|-
    // O|-|-
    gameboard.place_tile(&(0, 0), BoardStates::X);
    gameboard.place_tile(&(0, 1), BoardStates::X);
    gameboard.place_tile(&(1, 0), BoardStates::X);
    gameboard.place_tile(&(2, 0), BoardStates::O);

    let series = gameboard.check_if_two_in_series(&(0, 0));

    assert_eq!(series, expected_coordinates);
  }

  #[test]
  fn corner_check_over_center_with_fake_near() {
    let mut gameboard = BoardConfig::new();
    let expected_coordinates = Some((1, 1));

    // X|-|-
    // O|-|-
    // -|-|X
    gameboard.place_tile(&(0, 0), BoardStates::X);
    gameboard.place_tile(&(2, 2), BoardStates::X);
    gameboard.place_tile(&(1, 0), BoardStates::O);

    let series = gameboard.check_if_two_in_series(&(0, 0));

    assert_eq!(series, expected_coordinates);
  }

  #[test]
  fn edge_with_fake_corner_check_and_over_center() {
    let mut gameboard = BoardConfig::new();
    let expected_coordinates = Some((1, 1));

    // X|-|-
    // X|-|X
    // O|-|-
    gameboard.place_tile(&(0, 0), BoardStates::X);
    gameboard.place_tile(&(1, 0), BoardStates::X);
    gameboard.place_tile(&(1, 2), BoardStates::X);
    gameboard.place_tile(&(2, 0), BoardStates::O);

    let series = gameboard.check_if_two_in_series(&(1, 0));

    assert_eq!(series, expected_coordinates);
  }

  #[test]
  fn edge_from_corner_fake_over_center() {
    let mut gameboard = BoardConfig::new();
    let expected_coordinates = Some((2, 0));

    // X|-|-
    // X|O|X
    // -|-|-
    gameboard.place_tile(&(0, 0), BoardStates::X);
    gameboard.place_tile(&(1, 0), BoardStates::X);
    gameboard.place_tile(&(1, 2), BoardStates::X);
    gameboard.place_tile(&(1, 1), BoardStates::O);

    let series = gameboard.check_if_two_in_series(&(1, 0));

    assert_eq!(series, expected_coordinates);
  }
}
