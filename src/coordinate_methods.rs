use crate::gameboard::*;
use crate::gameplay::*;

pub type Coordinates = (usize, usize);

pub trait CoordinateMethods {
  fn get_opposite_coordinates(&self, adjacent_coords: &Coordinates) -> Coordinates;

  fn is_diagonal_from_return_self(&self, origin_coords: &Coordinates) -> Option<Coordinates>;
  fn is_across_from_return_self(&self, origin_coords: &Coordinates) -> Option<Coordinates>;
  fn is_diagonal_from(&self, origin_coords: &Coordinates) -> bool;
  fn is_across_from(&self, origin_coords: &Coordinates) -> bool;

  fn is_matching_in_a_row(&self, adjacent_coords: &Coordinates, board_config: &BoardConfig)
    -> bool;
  fn is_inbetween_matching(
    &self,
    adjacent_coords: &Coordinates,
    board_config: &BoardConfig,
  ) -> Option<bool>;
  fn is_side_matching(
    &self,
    adjacent_coords: &Coordinates,
    board_config: &BoardConfig,
  ) -> Option<bool>;
}

impl CoordinateMethods for Coordinates {
  fn get_opposite_coordinates(&self, adjacent_coords: &Coordinates) -> Coordinates {
    let coords: (isize, isize) = (
      adjacent_coords.0.try_into().unwrap(),
      adjacent_coords.1.try_into().unwrap(),
    );

    let origin_coords: (isize, isize) = (
      self.0.try_into().unwrap(), //
      self.1.try_into().unwrap(),
    );

    (
      (coords.0 - (origin_coords.0 - coords.0))
        .try_into()
        .unwrap(),
      (coords.1 - (origin_coords.1 - coords.1))
        .try_into()
        .unwrap(),
    )
  }

  fn is_diagonal_from_return_self(&self, origin_coords: &Coordinates) -> Option<Coordinates> {
    let isize_coordinates: [isize; 2] = [
      origin_coords.0.try_into().unwrap(),
      origin_coords.1.try_into().unwrap(),
    ];

    let possible_coordinates: Vec<(isize, isize)> = vec![
      (isize_coordinates[0] + 1, isize_coordinates[1] + 1),
      (isize_coordinates[0] - 1, isize_coordinates[1] + 1),
      (isize_coordinates[0] + 1, isize_coordinates[1] - 1),
      (isize_coordinates[0] - 1, isize_coordinates[1] - 1),
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

      let usize_coordinates = (
        coordinates.0.try_into().unwrap(),
        coordinates.1.try_into().unwrap(),
      );

      if &usize_coordinates == self {
        return Some(usize_coordinates);
      };
    }

    None
  }

  fn is_across_from_return_self(&self, origin_coords: &Coordinates) -> Option<Coordinates> {
    let isize_coordinates: [isize; 2] = [
      origin_coords.0.try_into().unwrap(),
      origin_coords.1.try_into().unwrap(),
    ];

    let possible_coordinates: Vec<(isize, isize)> = vec![
      (isize_coordinates[0] + 1, isize_coordinates[1]),
      (isize_coordinates[0] - 1, isize_coordinates[1]),
      (isize_coordinates[0], isize_coordinates[1] + 1),
      (isize_coordinates[0], isize_coordinates[1] - 1),
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

      let usize_coordinates = (
        coordinates.0.try_into().unwrap(),
        coordinates.1.try_into().unwrap(),
      );

      if &usize_coordinates == self {
        return Some(usize_coordinates);
      };
    }

    None
  }

  fn is_diagonal_from(&self, origin_coords: &Coordinates) -> bool {
    let isize_origin_coords: [isize; 2] = [
      origin_coords.0.try_into().unwrap(),
      origin_coords.1.try_into().unwrap(),
    ];

    let possible_coordinates: Vec<(isize, isize)> = vec![
      (isize_origin_coords[0] + 1, isize_origin_coords[1] + 1),
      (isize_origin_coords[0] - 1, isize_origin_coords[1] + 1),
      (isize_origin_coords[0] + 1, isize_origin_coords[1] - 1),
      (isize_origin_coords[0] - 1, isize_origin_coords[1] - 1),
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

      let usize_coordinates = (
        coordinates.0.try_into().unwrap(),
        coordinates.1.try_into().unwrap(),
      );

      if &usize_coordinates == self {
        return true;
      };
    }

    false
  }

  fn is_across_from(&self, origin_coords: &Coordinates) -> bool {
    let isize_coordinates: [isize; 2] = [
      origin_coords.0.try_into().unwrap(),
      origin_coords.1.try_into().unwrap(),
    ];

    let possible_coordinates: Vec<(isize, isize)> = vec![
      (isize_coordinates[0] + 1, isize_coordinates[1]),
      (isize_coordinates[0] - 1, isize_coordinates[1]),
      (isize_coordinates[0], isize_coordinates[1] + 1),
      (isize_coordinates[0], isize_coordinates[1] - 1),
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

      let usize_coordinates = (
        coordinates.0.try_into().unwrap(),
        coordinates.1.try_into().unwrap(),
      );

      if &usize_coordinates == self {
        return true;
      };
    }

    false
  }

  fn is_matching_in_a_row(
    &self,
    adjacent_coords: &Coordinates,
    board_config: &BoardConfig,
  ) -> bool {
    if let Some(x) = self.is_inbetween_matching(adjacent_coords, board_config) {
      x
    } else if let Some(x) = self.is_side_matching(adjacent_coords, board_config) {
      x
    } else {
      false
    }
  }

  fn is_inbetween_matching(
    &self,
    adjacent_coords: &Coordinates,
    board_config: &BoardConfig,
  ) -> Option<bool> {
    if board_config.get_board_position(self) == &BoardPositions::Center {
      let opposite_coords = adjacent_coords.get_opposite_coordinates(self);

      Some(board_config.get_board_state(&opposite_coords) == board_config.get_board_state(self))
    } else if board_config.get_board_position(self) == &BoardPositions::Edge
      && board_config.get_board_position(adjacent_coords) == &BoardPositions::Center
    {
      let opposite_coords = self.get_opposite_coordinates(adjacent_coords);

      Some(board_config.get_board_state(&opposite_coords) == board_config.get_board_state(self))
    } else {
      None
    }
  }

  fn is_side_matching(
    &self,
    adjacent_coords: &Coordinates,
    board_config: &BoardConfig,
  ) -> Option<bool> {
    if board_config.get_board_position(adjacent_coords) != &BoardPositions::Corner
      && board_config.get_board_position(adjacent_coords) != board_config.get_board_position(self)
    {
      let opposite_coords = self.get_opposite_coordinates(adjacent_coords);

      Some(board_config.get_board_state(&opposite_coords) == board_config.get_board_state(self))
    } else {
      None
    }
  }
}

#[cfg(test)]
mod coordinate_methods {
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

    if let Some(d) = diagonal.is_diagonal_from_return_self(&origin) {
      assert_eq!(d, diagonal);
    }

    if let Some(h) = horizontal.is_across_from_return_self(&origin) {
      assert_eq!(h, horizontal);
    }

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
