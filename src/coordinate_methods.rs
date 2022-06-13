use crate::gameboard::*;
use crate::gameplay::*;

pub type Coordinates = (usize, usize);

trait CoordinateMethods {
  fn calculate_opposing_coordinates(self, adjacent_coords: Coordinates) -> Option<Self>
  where
    Self: Sized;

  fn is_diagonal_from_origin(self, adjacent_coords: Coordinates) -> Option<(isize, isize)>;
  fn is_across_from_origin(self, adjacent_coords: Coordinates) -> Option<(isize, isize)>;
}

impl CoordinateMethods for Coordinates {
  fn calculate_opposing_coordinates(self, adjacent_coords: Coordinates) -> Option<Self>
  where
    Self: Sized,
  {
    let coords: (isize, isize) = (
      adjacent_coords.0.try_into().unwrap(),
      adjacent_coords.1.try_into().unwrap(),
    );

    let origin_coords: (isize, isize) = (
      self.0.try_into().unwrap(), //
      self.1.try_into().unwrap(),
    );

    Some((
      (coords.0 - (origin_coords.0 - coords.0))
        .try_into()
        .unwrap(),
      (coords.1 - (origin_coords.1 - coords.1))
        .try_into()
        .unwrap(),
    ))
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

  fn is_across_from_origin(self, adjacent_coords: Coordinates) -> Option<(isize, isize)> {
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
mod coordinate_methods {
  use super::*;

  #[test]
  fn calculate_opposing_coordinates_logic_works() {
    let origin = (0, 0);
    let adjacent_edge_1 = (0, 1);
    let adjacent_edge_2 = (1, 0);

    let opposite_1 = origin.calculate_opposing_coordinates(adjacent_edge_1);
    let opposite_2 = origin.calculate_opposing_coordinates(adjacent_edge_2);
    let opposite_3 = origin.calculate_opposing_coordinates((1, 1));

    assert_eq!(opposite_1, Some((0, 2)));
    assert_eq!(opposite_2, Some((2, 0)));
    assert_eq!(opposite_3, Some((2, 2)));
  }

  #[test]
  fn is_diagonal_and_horizontal_logic_works() {
    let origin = (0, 0);
    let diagonal = (1, 1);
    let horizontal = (0, 1);

    if let Some(x) = origin.is_diagonal_from_origin(diagonal) {
      println!("x is {:?}", x);
    }

    if let Some(x) = origin.is_across_from_origin(horizontal) {
      println!("x is {:?}", x);
    }
  }
}
