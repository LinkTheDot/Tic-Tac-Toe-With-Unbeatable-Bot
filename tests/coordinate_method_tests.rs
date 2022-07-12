use tictactoe_with_ai::coordinate_methods::*;
use tictactoe_with_ai::gameboard::*;

#[cfg(test)]
mod opposing_coordinates_logic {
  use super::*;

  #[test]
  fn corner_over_edge() {
    let origin = (0, 0);
    let adjacent_edge_1 = (0, 1);
    let adjacent_edge_2 = (1, 0);

    let expected_corner_1 = (0, 2);
    let expected_corner_2 = (2, 0);

    let opposite_1 = origin.get_opposite_coordinates(&adjacent_edge_1);
    let opposite_2 = origin.get_opposite_coordinates(&adjacent_edge_2);

    assert_eq!(opposite_1, expected_corner_1);
    assert_eq!(opposite_2, expected_corner_2);
  }

  #[test]
  fn corner_over_center() {
    let origin = (0, 0);
    let center = (1, 1);

    let expected_corner = (2, 2);

    let opposite_3 = origin.get_opposite_coordinates(&center);

    assert_eq!(opposite_3, expected_corner);
  }

  #[test]
  fn edge_over_center() {
    let origin = (1, 0);
    let center = (1, 1);

    let expected_edge = (1, 2);

    let opposite_3 = origin.get_opposite_coordinates(&center);

    assert_eq!(opposite_3, expected_edge);
  }
}

#[cfg(test)]
mod is_across_and_diagonal_logic {
  use super::*;

  #[test]
  fn is_accross_works() {
    let origin = (0, 0);
    let horizontal = (0, 1);
    let bad_input = (3, 3);

    let not_horizontal_from = bad_input.is_across_from(&origin);
    let horizontal_from = horizontal.is_across_from(&origin);

    assert_eq!(horizontal_from, true);
    assert_eq!(not_horizontal_from, false);
  }

  #[test]
  fn is_diagonal_works() {
    let origin = (0, 0);
    let diagonal = (1, 1);
    let bad_input = (3, 3);

    let diagonal_from = diagonal.is_diagonal_from(&origin);
    let not_diagonal_from = bad_input.is_diagonal_from(&origin);

    assert_eq!(diagonal_from, true);
    assert_eq!(not_diagonal_from, false);
  }
}

#[cfg(test)]
mod get_coords_around_logic {
  use super::*;

  #[test]
  fn from_corner() {
    let corner_coords = (0, 0);

    let around_corner = corner_coords.get_coords_around();
    let expected_coords_around = vec![(1, 0), (0, 1), (1, 1)];

    assert_eq!(around_corner, expected_coords_around);
  }

  #[test]
  fn from_edge() {
    let edge_coords = (1, 0);

    let expected_coords_around = vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)].sort();
    let around_edge = edge_coords.get_coords_around().sort();

    assert_eq!(around_edge, expected_coords_around);
  }
}

#[cfg(test)]
mod is_matching_in_a_row_logic {
  use super::*;

  const EXPECTED_NO_WIN: bool = false;
  const EXPECTED_WIN: bool = true;

  #[test]
  fn connected_to_non_matching_symbols_side() {
    let mut gameboard = BoardConfig::new();
    let check_from = (0, 0);
    let check_over = (1, 0);

    // X|-|-
    // O|-|-
    // O|-|-
    gameboard.place_tile(&(0, 0), BoardStates::X);
    gameboard.place_tile(&(1, 0), BoardStates::O);
    gameboard.place_tile(&(2, 0), BoardStates::O);

    let matching_result = check_from.is_matching_in_a_row(&check_over, &gameboard);

    assert_eq!(matching_result, EXPECTED_NO_WIN);
  }

  #[test]
  fn connected_to_match_check_from_side() {
    let mut gameboard = BoardConfig::new();
    let check_from = (0, 0);
    let check_over = (1, 0);

    // X|-|-
    // X|-|-
    // X|-|-
    gameboard.place_tile(&(0, 0), BoardStates::X);
    gameboard.place_tile(&(1, 0), BoardStates::X);
    gameboard.place_tile(&(2, 0), BoardStates::X);

    let matching_result = check_from.is_matching_in_a_row(&check_over, &gameboard);

    assert_eq!(matching_result, EXPECTED_WIN);
  }

  #[test]
  fn connected_to_match_check_from_in_between() {
    let mut gameboard = BoardConfig::new();
    let check_from = (1, 0);
    let check_over = (0, 0);

    // X|-|-
    // X|-|-
    // X|-|-
    gameboard.place_tile(&(0, 0), BoardStates::X);
    gameboard.place_tile(&(1, 0), BoardStates::X);
    gameboard.place_tile(&(2, 0), BoardStates::X);

    let matching_result = check_from.is_matching_in_a_row(&check_over, &gameboard);

    assert_eq!(matching_result, EXPECTED_WIN);
  }

  #[test]
  fn connected_to_possible_panic() {
    let mut gameboard = BoardConfig::new();
    let check_from = (1, 0);
    let check_over = (0, 1);

    // -|X|-
    // X|-|-
    // -|-|-
    gameboard.place_tile(&(1, 0), BoardStates::X);
    gameboard.place_tile(&(0, 1), BoardStates::X);

    let matching_result = check_from.is_matching_in_a_row(&check_over, &gameboard);

    assert_eq!(matching_result, EXPECTED_NO_WIN);
  }

  #[test]
  fn checking_from_center_possible_overflow() {
    let mut gameboard = BoardConfig::new();
    let check_from = (1, 1);
    let check_over = (2, 1);

    //-|O|-
    //-|X|-
    //-|X|-
    gameboard.place_tile(&(0, 1), BoardStates::O);
    gameboard.place_tile(&(1, 1), BoardStates::X);
    gameboard.place_tile(&(2, 1), BoardStates::X);

    let matching_result = check_from.is_matching_in_a_row(&check_over, &gameboard);

    assert_eq!(matching_result, EXPECTED_NO_WIN);
  }
}
