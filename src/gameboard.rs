use crate::coordinate_methods::*;
use crate::gameplay::GameConfig;

#[derive(PartialEq, Clone, Debug)]
pub struct BoardConfig {
  pub tiles: [[BoardTile; 3]; 3],
  pub tiles_covered: u8,
  pub player_symbol: BoardStates,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BoardTile {
  pub board_state: BoardStates,
  pub board_position: BoardPositions,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BoardStates {
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
      player_symbol: BoardStates::Empty,
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
    let matching_symbol: &BoardStates = {
      let symbol = &self.tiles[coords.0][coords.1].board_state;

      if symbol == &BoardStates::Empty {
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
    let origin_position = &self.tiles[coordinates.0][coordinates.1].board_position;
    let adjacent_matches = self.matching_adjacent_tiles(coordinates);

    adjacent_matches
      .iter()
      .filter(|coords| coordinates.is_matching_in_a_row(coords, self))
      .count()
      != 0
  }

  pub fn get_board_position(&self, coords: &Coordinates) -> &BoardPositions {
    &self.tiles[coords.0][coords.1].board_position
  }

  pub fn get_board_state(&self, coords: &Coordinates) -> &BoardStates {
    &self.tiles[coords.0][coords.1].board_state
  }
}

impl BoardTile {
  pub fn new(board_position: BoardPositions) -> Self {
    BoardTile {
      board_state: BoardStates::Empty,
      board_position,
    }
  }

  pub fn board_state_to_string(&self) -> String {
    match self.board_state {
      BoardStates::X => "X".to_string(),
      BoardStates::O => "O".to_string(),
      BoardStates::Empty => "▮".to_string(),
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
