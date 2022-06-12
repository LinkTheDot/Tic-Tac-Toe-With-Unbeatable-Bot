use crate::gameboard::*;
use crate::gameplay::*;

pub type Coordinates = (usize, usize);

trait CoordinateMethods {
  fn calculate_opposing_coordinates(
    origin_coords: Coordinates,
    board_position: BoardPositions,
    adjacent_coords: Coordinates,
  ) -> Option<Self>
  where
    Self: Sized;

  fn is_diagonal_from_origin(self, adjacent_coords: Coordinates) -> Option<Coordinates>;
  fn is_horizontal_from_origin(self, adjacent_coords: Coordinates) -> Option<Coordinates>;
}

impl CoordinateMethods for Coordinates {
  fn calculate_opposing_coordinates(
    origin_coords: Coordinates,
    board_position: BoardPositions,
    adjacent_coords: Coordinates,
  ) -> Option<Self>
  where
    Self: Sized,
  {
    match board_position {
      BoardPositions::Corner => {
        if let Some(coords) = origin_coords.is_diagonal_from_origin(adjacent_coords) {
          if (coords.0 + 2, coords.1 + 2) == (2, 2) {
            //sends bottom right
            Some((2, 2))
          } else if (coords.0, coords.1 + 2) == (2, 2) {
            //sends bottom left
            Some((0, 2))
          } else if (coords.0 + 2, coords.1) == (2, 2) {
            //sends top right
            Some((2, 0))
          } else if (coords.0, coords.1) == (2, 2) {
            //sends top left
            Some((0, 0))
          } else {
            None
          }

          // 0, 0 | 1, 0 | 2, 0
          // 0, 1 | 1, 1 | 2, 1
          // 0, 2 | 1, 2 | 2, 2
        } else if let Some(coords) = origin_coords.is_horizontal_from_origin(adjacent_coords) {
          // Horizontal/Vertical code
          None
        } else {
          None
        }
      }
      BoardPositions::Edge => None,
      _ => None,
    }
  }

  fn is_diagonal_from_origin(self, adjacent_coords: Coordinates) -> Option<Coordinates> {
    let isize_coordinates: [isize; 2] = [self.0.try_into().unwrap(), self.1.try_into().unwrap()];

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

      if usize_coordinates == adjacent_coords {
        return Some(usize_coordinates);
      };
    }

    None
  }

  fn is_horizontal_from_origin(self, adjacent_coords: Coordinates) -> Option<Coordinates> {
    let isize_coordinates: [isize; 2] = [self.0.try_into().unwrap(), self.1.try_into().unwrap()];

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

      if usize_coordinates == adjacent_coords {
        return Some(usize_coordinates);
      };
    }

    None
  }
}
