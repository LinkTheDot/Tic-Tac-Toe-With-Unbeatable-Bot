use tictactoe_with_ai::coordinate_methods::*;
use tictactoe_with_ai::gameboard::*;

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

#[test]
fn get_coords_around_logic_works() {
  let edge_coords = (1, 0);
  let corner_coords = (0, 0);

  let around_corner = corner_coords.get_coords_around();
  let around_edge = edge_coords.get_coords_around();

  println!("-corner coords-\n{:#?}", around_corner);
  println!("\n\n\n\n");
  println!("-edge coords-\n{:#?}", around_edge);
}
