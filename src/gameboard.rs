use crate::coordinate_methods::*;
use crate::gameplay::GameConfig;

#[derive(PartialEq, Clone, Debug)]
pub struct BoardConfig {
  pub tiles: [[BoardTile; 3]; 3],
  pub tiles_covered: u8,
  pub player_symbol: BoardState,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BoardTile {
  pub board_state: BoardState,
  pub board_position: BoardPositions,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BoardState {
  X,
  O,
  Empty,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BoardPositions {
  Corner,
  Edge,
  Center,
}

impl BoardConfig {
  pub fn new() -> Self {
    let tiles = [
      [
        BoardTile::new(BoardPositions::Corner),
        BoardTile::new(BoardPositions::Edge),
        BoardTile::new(BoardPositions::Corner),
      ],
      [
        BoardTile::new(BoardPositions::Edge),
        BoardTile::new(BoardPositions::Center),
        BoardTile::new(BoardPositions::Edge),
      ],
      [
        BoardTile::new(BoardPositions::Corner),
        BoardTile::new(BoardPositions::Edge),
        BoardTile::new(BoardPositions::Corner),
      ],
    ];

    BoardConfig {
      tiles,
      tiles_covered: 0,
      player_symbol: BoardState::Empty,
    }
  }

  pub fn all_tiles_covered(self) -> bool {
    self.tiles_covered == 9
  }

  pub fn print_board(&self) {
    for x in 0..3 {
      println!(
        "{}|{}|{}",
        self.tiles[x][0].board_state_to_string(),
        self.tiles[x][1].board_state_to_string(),
        self.tiles[x][2].board_state_to_string(),
      );
    }
  }

  pub fn matching_adjacent_tiles(&self, coords: Coordinates) -> Vec<Coordinates> {
    let adjacent_tiles = get_valid_coordinates_around(coords);
    let matching_symbol: &BoardState = {
      let symbol = &self.tiles[coords.0][coords.1].board_state;

      if symbol == &BoardState::Empty {
        return vec![];
      } else {
        symbol
      }
    };

    adjacent_tiles
      .iter()
      .filter(|x| &self.tiles[x.0][x.1].board_state == matching_symbol)
      .cloned()
      .collect::<Vec<Coordinates>>()
  }

  pub fn coordinates_connected_to_three_in_a_row(&self, coordinates: Coordinates) -> bool {
    // if the coord tile is a corner and has an adjacent tile of same symbol
    // find the opposing tile of that direction through some logic check all 3
    //
    // use an equation to find the directions
    //
    // if coord tile is edge and adjacent is center, find opposing edge through some logic
    // and check all 3
    //
    // if is edge and adjacent is corner, use same as corner logic to determine
    // the opposing corner and check all 3

    // - first find what position it is in
    // - next determine where an adjacent is
    // - iterate through them just incase there's multiple
    // - iteration logic - (start a counter for adjacent matching symbols)
    // - first if it's a corner OR edge that requires an opposing, find some algorithm
    // to find that (probably use a separate implementation for it)
    // - next if it's an edge that goes to a corner OR a center, take a direction and
    // start from there instead then run step one
    // - check if that counter from the start == 3 if not start the iteration over
    // and keep trying until you run out of things to iterate through

    let origin_position = &self.tiles[coordinates.0][coordinates.1].board_position;
    let adjacent_matches = self.matching_adjacent_tiles(coordinates);

    let _x = adjacent_matches.iter().map(|coords| coords);

    false
  }
}

impl BoardTile {
  pub fn new(board_position: BoardPositions) -> Self {
    BoardTile {
      board_state: BoardState::Empty,
      board_position,
    }
  }

  pub fn board_state_to_string(&self) -> String {
    match self.board_state {
      BoardState::X => "X".to_string(),
      BoardState::O => "O".to_string(),
      BoardState::Empty => "▮".to_string(),
    }
  }
}

pub fn get_valid_coordinates_around(coordinates: Coordinates) -> Vec<Coordinates> {
  let mut valid_coordinates: Vec<Coordinates> = Vec::new();
  let isize_coordinates = [
    coordinates.0.try_into().unwrap(),
    coordinates.1.try_into().unwrap(),
  ];

  let possible_coordinates: Vec<(isize, isize)> = vec![
    (isize_coordinates[0], isize_coordinates[1] - 1),
    (isize_coordinates[0], isize_coordinates[1] + 1),
    (isize_coordinates[0] - 1, isize_coordinates[1]),
    (isize_coordinates[0] + 1, isize_coordinates[1]),
    (isize_coordinates[0] - 1, isize_coordinates[1] + 1),
    (isize_coordinates[0] + 1, isize_coordinates[1] - 1),
    (isize_coordinates[0] - 1, isize_coordinates[1] - 1),
    (isize_coordinates[0] + 1, isize_coordinates[1] + 1),
  ];

  for coordinates in possible_coordinates {
    match coordinates.0 {
      -1 => continue,
      3 => continue,
      _ => (),
    }

    match coordinates.1 {
      -1 => continue,
      3 => continue,
      _ => (),
    }

    let swapped_to_usize: Coordinates = (
      coordinates.0.try_into().unwrap(),
      coordinates.1.try_into().unwrap(),
    );

    valid_coordinates.push(swapped_to_usize);
  }

  valid_coordinates
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
      let coords = get_valid_coordinates_around(dummy_coords);

      corner_coords_around_lengths.push(coords.len());
    }

    for dummy_coords in edge_dummy_coords {
      let coords = get_valid_coordinates_around(dummy_coords);

      edge_coords_around_lengths.push(coords.len());
    }

    {
      let coords = get_valid_coordinates_around(center_dummy_coords);

      center_coords_around_length = coords.len();
    }

    assert_eq!(vec![3, 3, 3, 3], corner_coords_around_lengths);
    assert_eq!(vec![5, 5, 5, 5], edge_coords_around_lengths);
    assert_eq!(8, center_coords_around_length);
  }

  #[test]
  pub fn matching_adjacent_tiles_logic_works() {
    let mut board_config = BoardConfig::new();

    board_config.tiles[0][0].board_state = BoardState::X;
    board_config.tiles[1][0].board_state = BoardState::X;

    let adjacent_matched_tiles = board_config.matching_adjacent_tiles((0, 0));
    let adjacent_empty_tile = board_config.matching_adjacent_tiles((1, 1));
    let empty_vec_because_bugged: Vec<Coordinates> = vec![];

    // checks if it returns real matching symbols
    assert_eq!(vec![(1, 0)], adjacent_matched_tiles);
    // checks if it returns nothing when the tile symbol is Empty
    assert_eq!(empty_vec_because_bugged, adjacent_empty_tile);
  }
}
