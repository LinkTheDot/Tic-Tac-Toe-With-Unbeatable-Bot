use crate::coordinate_methods::*;
use crate::gameplay::GameConfig;
use rand::prelude::*;

#[derive(PartialEq, Debug)]
pub struct BoardConfig {
  pub tiles: [[BoardTile; 3]; 3],
  pub tiles_covered: u8,
  pub player_symbol: BoardStates,
  pub last_modified_tile: Coordinates,
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
      last_modified_tile: (10, 10),
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

  pub fn matching_adjacent_tiles(&self, coords: &Coordinates) -> Vec<Coordinates> {
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
      .filter(|coords| self.get_board_state(coords) == matching_symbol)
      .cloned()
      .collect::<Vec<Coordinates>>()
  }

  // remove coordinates and replace with self.last_modified_tile if bot doesn't use
  pub fn coordinates_connected_to_three_in_a_row(&self, coordinates: &Coordinates) -> bool {
    let origin_position = &self.get_board_position(coordinates);
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

  pub fn check_corner_states(&self) -> [(Coordinates, &BoardStates); 4] {
    let top_left = (0, 0);
    let top_right = (0, 2);
    let bottom_left = (2, 0);
    let bottom_right = (2, 2);

    [
      (top_left, self.get_board_state(&top_left)),
      (top_right, self.get_board_state(&top_right)),
      (bottom_left, self.get_board_state(&bottom_left)),
      (bottom_right, self.get_board_state(&bottom_right)),
    ]
  }

  pub fn check_edge_states(&self) -> [(Coordinates, &BoardStates); 4] {
    let top = (0, 1);
    let left = (1, 0);
    let right = (1, 2);
    let bottom = (2, 1);

    [
      (top, self.get_board_state(&top)),
      (left, self.get_board_state(&left)),
      (right, self.get_board_state(&right)),
      (bottom, self.get_board_state(&bottom)),
    ]
  }

  pub fn place_tile(&mut self, coords: Coordinates, changed_state: BoardStates) {
    self.tiles[coords.0][coords.1].board_state = changed_state;
  }

  pub fn get_random_empty_corner(&self) -> Option<Coordinates> {
    let corners: Vec<Coordinates> = vec![(0, 0), (2, 0), (0, 2), (2, 2)];

    let mut valid_corners: Vec<Coordinates> = corners
      .into_iter()
      .filter(|coords| self.get_board_state(&coords) == &BoardStates::Empty)
      .collect::<Vec<Coordinates>>();

    if valid_corners.len() != 0 {
      Some(valid_corners[rand::thread_rng().gen_range(0..valid_corners.len())])
    } else {
      None
    }
  }

  pub fn get_random_empty_edge(&self) -> Option<Coordinates> {
    let edges: Vec<Coordinates> = vec![(0, 1), (1, 0), (1, 2), (2, 1)];

    let mut valid_corners: Vec<Coordinates> = edges
      .into_iter()
      .filter(|coords| self.get_board_state(&coords) == &BoardStates::Empty)
      .collect();

    if valid_corners.len() != 0 {
      Some(valid_corners[rand::thread_rng().gen_range(0..valid_corners.len())])
    } else {
      None
    }
  }

  /// If there is a series of 2, this will return the empty one in the series.
  /// Otherwise it'll return None.
  pub fn check_if_two_in_series(&self, check_from: &Coordinates) -> Option<Coordinates> {
    let checking_state = self.get_board_state(check_from);

    let near_coords: Vec<Coordinates> = check_from
      .get_coords_around()
      .into_iter()
      .filter(|coords| self.get_board_position(&coords) != self.get_board_position(&check_from))
      .collect();

    match self.get_board_position(check_from) {
      BoardPositions::Edge => {
        let from_corner: Vec<Coordinates> = near_coords
          .into_iter()
          .filter(|coords| {
            self.get_board_position(&coords) == &BoardPositions::Corner
              && self.get_board_state(&coords) == checking_state
              && self.get_board_state(&coords.get_opposite_coordinates(&check_from))
                == &BoardStates::Empty
          })
          .collect();

        let from_edge: Vec<Coordinates> = vec![
          *check_from,
          (1, 1),
          check_from.get_opposite_coordinates(&(1, 1)),
        ]
        .into_iter()
        .filter(|coords| self.get_board_state(&coords) == &BoardStates::Empty)
        .collect();

        if from_corner.len() == 1 {
          Some(from_corner[0].get_opposite_coordinates(check_from))
        } else if from_edge.len() == 1 {
          Some(from_edge[0])
        } else {
          None
        }
      }
      BoardPositions::Corner => {
        let valid_empty_far_coords = near_coords
          .iter()
          .map(|coords| {
            if self.get_board_state(&coords) == checking_state
              && self.get_board_state(&check_from.get_opposite_coordinates(&coords))
                == &BoardStates::Empty
            {
              check_from.get_opposite_coordinates(&coords)
            } else {
              (10, 10)
            }
          })
          .filter(|coords| coords != &(10, 10))
          .collect::<Vec<Coordinates>>();

        let valid_empty_near_coords = near_coords
          .iter()
          .filter(|coords| {
            self.get_board_state(&coords) == &BoardStates::Empty
              && self.get_board_state(&check_from.get_opposite_coordinates(&coords))
                == checking_state
          })
          .collect::<Vec<&Coordinates>>();

        if valid_empty_far_coords.len() == 1 {
          Some(valid_empty_far_coords[0])
        } else if valid_empty_near_coords.len() == 1 {
          Some(*valid_empty_near_coords[0])
        } else {
          None
        }
      }
      _ => None,
    }
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

pub fn get_valid_coordinates_around(coordinates: &Coordinates) -> Vec<Coordinates> {
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
