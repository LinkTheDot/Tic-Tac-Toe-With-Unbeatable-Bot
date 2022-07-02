#[cfg(test)]
use crate::coordinate_methods::*;
use crate::gameboard::*;

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