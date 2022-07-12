use crate::gameboard::*;

pub const GRID_SIZE: usize = 3;
pub const ISIZE_GRID_SIZE: isize = 3;

pub type Coordinates = (usize, usize);

pub trait CoordinateMethods {
  fn get_opposite_coordinates(&self, adjacent_coords: &Coordinates) -> Coordinates;

  fn is_diagonal_from(&self, origin_coords: &Coordinates) -> bool;
  fn is_across_from(&self, origin_coords: &Coordinates) -> bool;

  fn is_matching_in_a_row(&self, adjacent_coords: &Coordinates, board_config: &BoardConfig)
    -> bool;
  fn is_in_between_matching(
    &self,
    adjacent_coords: &Coordinates,
    board_config: &BoardConfig,
  ) -> Option<bool>;

  fn is_side_matching(
    &self,
    adjacent_coords: &Coordinates,
    board_config: &BoardConfig,
  ) -> Option<bool>;

  fn get_coords_around_excluding_center(&self) -> Vec<Coordinates>;
  fn get_coords_around(&self) -> Vec<Coordinates>;
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
        _x if _x == ISIZE_GRID_SIZE => continue,
        _ => (),
      }

      match coordinates.1 {
        -1 => continue,
        _x if _x == ISIZE_GRID_SIZE => continue,
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
        _x if _x == ISIZE_GRID_SIZE => continue,
        _ => (),
      }

      match coordinates.1 {
        -1 => continue,
        _x if _x == ISIZE_GRID_SIZE => continue,
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

  // merge with coordinates_connected_to_three_in_a_row if bot doesn't use
  fn is_matching_in_a_row(
    &self,
    adjacent_coords: &Coordinates,
    board_config: &BoardConfig,
  ) -> bool {
    if let Some(is_matching) = self.is_in_between_matching(adjacent_coords, board_config) {
      is_matching
    } else if let Some(is_matching) = self.is_side_matching(adjacent_coords, board_config) {
      is_matching
    } else {
      false
    }
  }

  fn is_in_between_matching(
    &self,
    adjacent_coords: &Coordinates,
    board_config: &BoardConfig,
  ) -> Option<bool> {
    if board_config.get_board_position(self) == &BoardPositions::Center
      || board_config.get_board_position(self) == &BoardPositions::Edge
        && board_config.get_board_position(adjacent_coords) == &BoardPositions::Corner
    {
      let opposite_coords = adjacent_coords.get_opposite_coordinates(self);

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

  /// if you pass in an edge it'll return corners
  /// if you pass in a corner it'll return edges
  /// if you end up passing in the center then you'll get all edges
  fn get_coords_around_excluding_center(&self) -> Vec<Coordinates> {
    let isize_coordinates: [isize; 2] = [self.0.try_into().unwrap(), self.1.try_into().unwrap()];

    [
      (isize_coordinates[0] + 1, isize_coordinates[1]),
      (isize_coordinates[0] - 1, isize_coordinates[1]),
      (isize_coordinates[0], isize_coordinates[1] + 1),
      (isize_coordinates[0], isize_coordinates[1] - 1),
    ]
    .into_iter()
    .filter(|coords| {
      coords.0 != -1 && coords.0 != 3 && coords.1 != -1 && coords.1 != 3 && coords != &(1, 1)
    })
    .map(|coords| (coords.0.try_into().unwrap(), coords.1.try_into().unwrap()))
    .collect::<Vec<Coordinates>>()
  }

  fn get_coords_around(&self) -> Vec<Coordinates> {
    let isize_coordinates: [isize; 2] = [self.0.try_into().unwrap(), self.1.try_into().unwrap()];

    [
      (isize_coordinates[0] + 1, isize_coordinates[1]),
      (isize_coordinates[0] - 1, isize_coordinates[1]),
      (isize_coordinates[0], isize_coordinates[1] + 1),
      (isize_coordinates[0], isize_coordinates[1] - 1),
      (isize_coordinates[0] + 1, isize_coordinates[1] + 1),
      (isize_coordinates[0] - 1, isize_coordinates[1] - 1),
      (isize_coordinates[0] - 1, isize_coordinates[1] + 1),
      (isize_coordinates[0] + 1, isize_coordinates[1] - 1),
    ]
    .into_iter()
    .filter(|coords| coords.0 != -1 && coords.0 != 3 && coords.1 != -1 && coords.1 != 3)
    .map(|coords| (coords.0.try_into().unwrap(), coords.1.try_into().unwrap()))
    .collect::<Vec<Coordinates>>()
  }
}
