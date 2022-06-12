use crate::gameboard::*;
use crate::gameplay::*;

pub type Coordinates = (usize, usize);

trait CoordinateMethods {
  fn calculate_opposing_coordinates(
    self,
    adjacent_coords: Coordinates,
    board_position: BoardPositions,
  ) -> Option<Self>
  where
    Self: Sized;

  fn is_diagonal_from_origin(self, adjacent_coords: Coordinates) -> Option<(isize, isize)>;
  fn is_horizontal_from_origin(self, adjacent_coords: Coordinates) -> Option<(isize, isize)>;
}

impl CoordinateMethods for Coordinates {
  fn calculate_opposing_coordinates(
    self,
    adjacent_coords: Coordinates,
    board_position: BoardPositions,
  ) -> Option<Self>
  where
    Self: Sized,
  {
    match board_position {
      BoardPositions::Corner => {
        if let Some(coords) = self.is_diagonal_from_origin(adjacent_coords) {
          // corners diagonal opposite
          //    = x2, y2 + (x1, y1 - 1, 1)

          let origin_coords: (isize, isize) =
            (self.0.try_into().unwrap(), self.1.try_into().unwrap());

          Some((
            (coords.0 + (origin_coords.0 - 1)).try_into().unwrap(),
            (coords.1 + (origin_coords.1 - 1)).try_into().unwrap(),
          ))
        } else if let Some(coords) = self.is_horizontal_from_origin(adjacent_coords) {
          // opposite
          //    = x2, y2 + |(x1, y1 - x2, y2)|

          let origin_coords: (isize, isize) =
            (self.0.try_into().unwrap(), self.1.try_into().unwrap());

          Some((
            (coords.0 + (origin_coords.0 - coords.0).abs())
              .try_into()
              .unwrap(),
            (coords.1 + (origin_coords.1 - coords.1).abs())
              .try_into()
              .unwrap(),
          ))
        } else {
          None
        }
      }
      BoardPositions::Edge => {
        if let Some(coords) = self.is_horizontal_from_origin(adjacent_coords) {
          let origin_coords: (isize, isize) =
            (self.0.try_into().unwrap(), self.1.try_into().unwrap());

          Some((
            (coords.0 + (origin_coords.0 - coords.0).abs())
              .try_into()
              .unwrap(),
            (coords.1 + (origin_coords.1 - coords.1).abs())
              .try_into()
              .unwrap(),
          ))
        } else {
          None
        }
      }
      _ => None,
    }
  }

  fn is_diagonal_from_origin(self, adjacent_coords: Coordinates) -> Option<(isize, isize)> {
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
        return Some(coordinates);
      };
    }

    None
  }

  fn is_horizontal_from_origin(self, adjacent_coords: Coordinates) -> Option<(isize, isize)> {
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
        return Some(coordinates);
      };
    }

    None
  }
}

#[cfg(test)]
mod coord_tests {
  use super::*;

  #[test]
  fn calculate_opposing_coordinates_logic_works() {
    let origin = (0, 0);
    let adjacent_edge_1 = (0, 1);
    let adjacent_edge_2 = (1, 0);

    let opposite_1 = origin.calculate_opposing_coordinates(adjacent_edge_1, BoardPositions::Edge);
    let opposite_2 = origin.calculate_opposing_coordinates(adjacent_edge_2, BoardPositions::Edge);
    let opposite_3 = origin.calculate_opposing_coordinates((1, 1), BoardPositions::Corner);

    assert_eq!(opposite_1, Some((0, 2)));
    assert_eq!(opposite_2, Some((2, 0)));
    assert_eq!(opposite_3, Some((2, 2)));
  }
}
