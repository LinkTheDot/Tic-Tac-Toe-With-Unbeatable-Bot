use crate::coordinate_methods::*;
use crate::gameplay::GameState;
use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};

const VISUALIZED_X: &str = "X";
const VISUALIZED_O: &str = "O";
const VISUALIZED_EMPTY: &str = "▮";

#[derive(PartialEq, Debug)]
pub struct BoardConfig {
  pub tiles: [[BoardTile; GRID_SIZE]; GRID_SIZE],
  pub tiles_covered: u8,
  pub player_symbol: BoardStates,
  pub last_modified_tile: Option<Coordinates>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BoardTile {
  pub board_state: BoardStates,
  pub board_position: BoardPositions,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum BoardStates {
  X,
  O,
  Empty,
}

impl AsRef<BoardStates> for BoardStates {
  fn as_ref(&self) -> &Self {
    self
  }
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
      last_modified_tile: None,
    }
  }

  pub fn all_tiles_covered(self) -> bool {
    self.tiles_covered == 9
  }

  pub fn print_board(&self) {
    self
      .tiles
      .iter()
      .flatten()
      .map(|tile| format!("{}", tile))
      .collect::<Vec<String>>()
      .chunks(GRID_SIZE)
      .for_each(|row| println!("{}|{}|{}", row[0], row[1], row[2]));
  }

  /// this will return all coordinates around an input that are of the same boardstate
  pub fn matching_adjacent_tiles(&self, coords: &Coordinates) -> Vec<Coordinates> {
    let adjacent_tiles = coords.get_coords_around();
    let matching_symbol: &BoardStates = {
      let symbol = &self.get_board_state(coords);

      if *symbol == &BoardStates::Empty {
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

  pub fn get_board_position(&self, coords: &Coordinates) -> &BoardPositions {
    &self.tiles[coords.0][coords.1].board_position
  }

  pub fn get_board_state(&self, coords: &Coordinates) -> &BoardStates {
    &self.tiles[coords.0][coords.1].board_state
  }

  pub fn place_tile<B: AsRef<BoardStates>>(&mut self, coords: &Coordinates, changed_state: B) {
    self.last_modified_tile = Some(*coords);
    self.tiles[coords.0][coords.1].board_state = *changed_state.as_ref();
  }

  pub fn get_random_empty_corner(&self) -> Option<Coordinates> {
    let corners: Vec<Coordinates> = vec![(0, 0), (2, 0), (0, 2), (2, 2)];

    let valid_corners: Vec<Coordinates> = corners
      .into_iter()
      .filter(|coords| self.get_board_state(coords) == &BoardStates::Empty)
      .collect::<Vec<Coordinates>>();

    if !valid_corners.is_empty() {
      Some(valid_corners[rand::thread_rng().gen_range(0..valid_corners.len())])
    } else {
      None
    }
  }

  pub fn get_random_empty_edge(&self) -> Option<Coordinates> {
    let edges: Vec<Coordinates> = vec![(0, 1), (1, 0), (1, 2), (2, 1)];

    let valid_corners: Vec<Coordinates> = edges
      .into_iter()
      .filter(|coords| self.get_board_state(coords) == &BoardStates::Empty)
      .collect();

    if !valid_corners.is_empty() {
      Some(valid_corners[rand::thread_rng().gen_range(0..valid_corners.len())])
    } else {
      None
    }
  }

  /// returns a random empty corner then once there're no more empty corners
  /// it returns a random empty edge
  pub fn get_random_empty_corner_then_edge(&self) -> Option<Coordinates> {
    if let Some(coords) = self.get_random_empty_corner() {
      Some(coords)
    } else {
      self.get_random_empty_edge()
    }
  }

  /// If there is a series of 2, this will return the empty one in the series.
  /// Otherwise it'll return None.
  pub fn check_if_two_in_series(&self, check_from: &Coordinates) -> Option<Coordinates> {
    let nearby_coords: Vec<Coordinates> = check_from
      .get_coords_around()
      .into_iter()
      .filter(|coords| self.get_board_position(coords) != self.get_board_position(check_from))
      .collect();

    match self.get_board_position(check_from) {
      BoardPositions::Edge => series_of_two_edge_check(self, check_from, nearby_coords),
      BoardPositions::Corner => series_of_two_corner_check(self, check_from, nearby_coords),
      _ => None,
    }
  }

  pub fn last_placed_tile_to_game_state(&mut self) -> GameState {
    match self.get_board_state(&self.last_modified_tile.unwrap()) {
      BoardStates::X => GameState::XWon,
      BoardStates::O => GameState::OWon,
      _ => GameState::Draw,
    }
  }

  pub fn get_first_empty_tile(&self, coordinates: Vec<Coordinates>) -> Option<Coordinates> {
    coordinates.iter().find_map(|coords| {
      if self.get_board_state(coords) == &BoardStates::Empty {
        Some(*coords)
      } else {
        None
      }
    })
  }

  pub fn get_first_filled_tile(&self, coordinates: Vec<Coordinates>) -> Option<Coordinates> {
    coordinates.iter().find_map(|coords| {
      if self.get_board_state(coords) != &BoardStates::Empty {
        Some(*coords)
      } else {
        None
      }
    })
  }
}

impl BoardTile {
  pub fn new(board_position: BoardPositions) -> Self {
    BoardTile {
      board_state: BoardStates::Empty,
      board_position,
    }
  }
}

impl Display for BoardTile {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let output = match self.board_state {
      BoardStates::X => VISUALIZED_X,
      BoardStates::O => VISUALIZED_O,
      BoardStates::Empty => VISUALIZED_EMPTY,
    };

    write!(f, "{}", output)
  }
}

fn series_of_two_edge_check(
  gameboard: &BoardConfig,
  check_from: &Coordinates,
  nearby_coords: Vec<Coordinates>,
) -> Option<Coordinates> {
  let from_corner: Vec<Coordinates> = nearby_coords
    .into_iter()
    .filter(|coords| {
      gameboard.get_board_position(coords) == &BoardPositions::Corner
        && gameboard.get_board_state(coords) == gameboard.get_board_state(check_from)
        && gameboard.get_board_state(&coords.get_opposite_coordinates(check_from))
          == &BoardStates::Empty
    })
    .collect();

  let from_edge: Vec<Coordinates> = vec![
    *check_from,
    (1, 1),
    check_from.get_opposite_coordinates(&(1, 1)),
  ]
  .into_iter()
  .filter(|coords| gameboard.get_board_state(coords) == &BoardStates::Empty)
  .collect();

  if from_corner.len() == 1 {
    Some(from_corner[0].get_opposite_coordinates(check_from))
  } else if from_edge.len() == 1 {
    Some(from_edge[0])
  } else {
    None
  }
}

fn series_of_two_corner_check(
  gameboard: &BoardConfig,
  check_from: &Coordinates,
  nearby_coords: Vec<Coordinates>,
) -> Option<Coordinates> {
  let checking_state = gameboard.get_board_state(check_from);

  let valid_empty_far_coords = nearby_coords
    .iter()
    .filter_map(|coords| {
      if gameboard.get_board_state(coords) == checking_state
        && gameboard.get_board_state(&check_from.get_opposite_coordinates(coords))
          == &BoardStates::Empty
      {
        Some(check_from.get_opposite_coordinates(coords))
      } else {
        None
      }
    })
    .collect::<Vec<Coordinates>>();

  let valid_empty_near_coords = nearby_coords
    .iter()
    .filter(|coords| {
      gameboard.get_board_state(coords) == &BoardStates::Empty
        && gameboard.get_board_state(&check_from.get_opposite_coordinates(coords)) == checking_state
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
