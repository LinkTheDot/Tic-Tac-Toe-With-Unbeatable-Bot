use crate::gameboard::*;

pub const GRID_SIZE: usize = 3;
pub const ISIZE_GRID_SIZE: isize = 3;
const ISIZE_MIDDLE_COORDS: (isize, isize) = (1, 1);

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

  fn get_corners_around_edge(&self, gameboard: &BoardConfig) -> Vec<Coordinates>;
  fn get_edges_around_corner(&self, gameboard: &BoardConfig) -> Vec<Coordinates>;

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
    let isize_coords: [isize; 2] = [
      origin_coords.0.try_into().unwrap(),
      origin_coords.1.try_into().unwrap(),
    ];

    let isize_comparison: (isize, isize) = (self.0.try_into().unwrap(), self.1.try_into().unwrap());

    [
      (isize_coords[0] + 1, isize_coords[1] + 1),
      (isize_coords[0] - 1, isize_coords[1] + 1),
      (isize_coords[0] + 1, isize_coords[1] - 1),
      (isize_coords[0] - 1, isize_coords[1] - 1),
    ]
    .into_iter()
    .filter(|coords| coords == &isize_comparison)
    .count()
      != 0
  }

  fn is_across_from(&self, origin_coords: &Coordinates) -> bool {
    let isize_coordinates: [isize; 2] = [
      origin_coords.0.try_into().unwrap(),
      origin_coords.1.try_into().unwrap(),
    ];

    let isize_comparison: (isize, isize) = (self.0.try_into().unwrap(), self.1.try_into().unwrap());

    [
      (isize_coordinates[0] + 1, isize_coordinates[1]),
      (isize_coordinates[0] - 1, isize_coordinates[1]),
      (isize_coordinates[0], isize_coordinates[1] + 1),
      (isize_coordinates[0], isize_coordinates[1] - 1),
    ]
    .into_iter()
    .filter(|coords| coords == &isize_comparison)
    .count()
      != 0
  }

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

  fn get_corners_around_edge(&self, gameboard: &BoardConfig) -> Vec<Coordinates> {
    if gameboard.get_board_position(self) == &BoardPositions::Edge {
      get_corners_around_edge_and_edges_around_corner(self)
    } else {
      vec![]
    }
  }

  fn get_edges_around_corner(&self, gameboard: &BoardConfig) -> Vec<Coordinates> {
    if gameboard.get_board_position(self) == &BoardPositions::Corner {
      get_corners_around_edge_and_edges_around_corner(self)
    } else {
      vec![]
    }
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
    .filter(|coords| {
      coords.0 != -1 && coords.0 != ISIZE_GRID_SIZE && coords.1 != -1 && coords.1 != ISIZE_GRID_SIZE
    })
    .map(|coords| (coords.0.try_into().unwrap(), coords.1.try_into().unwrap()))
    .collect::<Vec<Coordinates>>()
  }
}

fn get_corners_around_edge_and_edges_around_corner(coordinates: &Coordinates) -> Vec<Coordinates> {
  let isize_coordinates: [isize; 2] = [
    coordinates.0.try_into().unwrap(),
    coordinates.1.try_into().unwrap(),
  ];

  [
    (isize_coordinates[0] + 1, isize_coordinates[1]),
    (isize_coordinates[0] - 1, isize_coordinates[1]),
    (isize_coordinates[0], isize_coordinates[1] + 1),
    (isize_coordinates[0], isize_coordinates[1] - 1),
  ]
  .into_iter()
  .filter(|coords| {
    coords.0 != -1
      && coords.0 != ISIZE_GRID_SIZE
      && coords.1 != -1
      && coords.1 != ISIZE_GRID_SIZE
      && coords != &ISIZE_MIDDLE_COORDS
  })
  .map(|coords| (coords.0.try_into().unwrap(), coords.1.try_into().unwrap()))
  .collect::<Vec<Coordinates>>()
}
