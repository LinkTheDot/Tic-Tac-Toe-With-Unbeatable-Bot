use crate::coordinate_methods::*;
use crate::gameboard::*;
use rand::prelude::*;

const CENTER_TILE: Coordinates = (1, 1);

#[derive(PartialEq, Debug)]
pub struct Bot {
  pub path: CurrentPath,
  pub bot_symbol: BoardStates,
  pub most_recent_chosen_coords: Result<Coordinates, String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CurrentPath {
  Center(BotCenterPaths),
  NotCenter(PlayerCenterPaths),
  DoubleWinCondition,
  FocusDraw,
  Unknown,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BotCenterPaths {
  PlayerPlacedEdge,
  PlayerPlacedCorner,
  Unknown,
}

#[derive(PartialEq, Clone, Debug)]
pub enum PlayerCenterPaths {
  PlayerDidntPlaceCenter,
  PlayerPlacedEdgeNear,
  Unknown,
}

//                   == LOGIC ==
//
// --FIRST TWO MOVES--
// if you get center and player places on an edge, win is Guaranteed
// if you get center and player places on corner, win is possible
// if you don't get center, never place on edge, always corner
//
// --PLAYER PLACES ON EDGE WITH YOU CENTER--
// place a piece on a corner next to player to force them to block your first 3 in a row
// place on edge next to corner you placed
// win
//
// --PLAYER PLACES ON CORNER WITH YOU CENTER--
// place on corner opposite of player
// if player places on an edge win is Guaranteed
// if player places on any corner, FocusDraw
//
// --PLAYER HAS CENTER--
// win is Medium
// place on corner
// if player places on opposite corner place on a corner and FocusDraw
// if player places on edge next to your piece get opposite and place there
//   - if from there player places on the corner over their edge from your corner,
//   win is Guaranteed
//   - otherwise FocusDraw
// if player places anywhere else FocusDraw

impl Bot {
  pub fn new() -> Self {
    Bot {
      path: CurrentPath::Unknown,
      bot_symbol: BoardStates::Empty,
      most_recent_chosen_coords: Err("No error has been given".to_string()),
    }
  }

  pub fn choose_coordinates(&mut self, gameboard: &BoardConfig) {
    match &self.path {
      CurrentPath::Center(_) => {
        center_position_checks(self, gameboard);
      }
      CurrentPath::NotCenter(_) => {
        not_center_position_checks(self, gameboard);
      }
      CurrentPath::FocusDraw | CurrentPath::DoubleWinCondition => {
        self.most_recent_chosen_coords = self.auto_play(gameboard);
      }
      CurrentPath::Unknown => {
        self.path = self.check_if_center_or_not(gameboard);

        if self.path == CurrentPath::Center(BotCenterPaths::Unknown) {
          self.most_recent_chosen_coords = Ok(CENTER_TILE);
        } else {
          self.most_recent_chosen_coords = self.initial_check_of_player_center_paths(gameboard);
        }
      }
    }
  }

  /// This will only be called in the first 2 moves
  pub fn check_if_center_or_not(&self, gameboard: &BoardConfig) -> CurrentPath {
    if gameboard.tiles_covered == 0 // /
      && gameboard.get_board_state(&CENTER_TILE) == &BoardStates::Empty
    {
      CurrentPath::Center(BotCenterPaths::Unknown)
    } else if gameboard.get_board_state(&CENTER_TILE) == &BoardStates::Empty {
      CurrentPath::NotCenter(PlayerCenterPaths::PlayerDidntPlaceCenter)
    } else {
      CurrentPath::NotCenter(PlayerCenterPaths::Unknown)
    }
  }

  /// This will only be called in the first 2 moves
  pub fn initial_check_of_player_center_paths(
    &mut self,
    gameboard: &BoardConfig,
  ) -> Result<Coordinates, String> {
    if let CurrentPath::NotCenter(path_state) = &self.path {
      match path_state {
        PlayerCenterPaths::PlayerDidntPlaceCenter => {
          self.path = CurrentPath::FocusDraw;

          Ok(CENTER_TILE)
        }
        PlayerCenterPaths::Unknown => {
          self.path = CurrentPath::FocusDraw;

          Ok(gameboard.get_random_empty_corner().unwrap())
        }
        _ => Err("neither 'Unknown' or 'DidntPlaceCenter'".to_string()),
      }
    } else {
      Err("non-NotCenter path has been called".to_string())
    }
  }

  pub fn not_center_corner_checks(
    &mut self,
    gameboard: &BoardConfig,
  ) -> Result<Coordinates, String> {
    match &self.path {
      CurrentPath::NotCenter(PlayerCenterPaths::Unknown) => {
        if let Ok(bot_tile) = &self.most_recent_chosen_coords {
          let corner_state_opposite_bot_corner =
            gameboard.get_board_state(&bot_tile.get_opposite_coordinates(&CENTER_TILE));

          self.path = CurrentPath::FocusDraw;

          if corner_state_opposite_bot_corner != &BoardStates::Empty {
            Ok(gameboard.get_random_empty_corner().unwrap())
          } else {
            self.auto_play(gameboard)
          }
        } else {
          Err("Last placed tile is empty".to_string())
        }
      }

      CurrentPath::NotCenter(PlayerCenterPaths::PlayerPlacedEdgeNear) => {
        not_center_corner_check_placed_edge_near(self, gameboard)
      }

      _ => Err("Unknown NotCenter Bot Path".to_string()),
    }
  }

  pub fn not_center_edge_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    let edge_coords_around_bot_corner = self
      .most_recent_chosen_coords
      .as_ref()
      .unwrap()
      .get_edges_around_corner(gameboard)
      .into_iter()
      .filter(|edge| edge == &gameboard.last_modified_tile)
      .collect::<Vec<Coordinates>>();

    if !edge_coords_around_bot_corner.is_empty() {
      self.path = CurrentPath::NotCenter(PlayerCenterPaths::PlayerPlacedEdgeNear);

      Ok(edge_coords_around_bot_corner[0].get_opposite_coordinates(&CENTER_TILE))
    } else {
      self.path = CurrentPath::FocusDraw;

      self.auto_play(gameboard)
    }
  }

  pub fn center_corner_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    match &self.path {
      CurrentPath::Center(BotCenterPaths::Unknown) => {
        self.path = CurrentPath::Center(BotCenterPaths::PlayerPlacedCorner);

        let opposite_coords = gameboard
          .last_modified_tile
          .get_opposite_coordinates(&CENTER_TILE);

        if gameboard.get_board_state(&opposite_coords) == &BoardStates::Empty {
          Ok(opposite_coords)
        } else {
          Err("Opposite corner is filled".to_string())
        }
      }
      CurrentPath::Center(BotCenterPaths::PlayerPlacedCorner) => {
        self.path = CurrentPath::FocusDraw;

        self.auto_play(gameboard)
      }
      CurrentPath::Center(BotCenterPaths::PlayerPlacedEdge) => {
        center_corner_check_placed_edge(self, gameboard)
      }
      _ => Err("Unknown Center Path".to_string()),
    }
  }

  pub fn center_edge_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    match &self.path {
      CurrentPath::Center(BotCenterPaths::Unknown) => {
        self.path = CurrentPath::Center(BotCenterPaths::PlayerPlacedEdge);

        let corners_near_player_edge = gameboard
          .last_modified_tile
          .get_corners_around_edge(gameboard);

        Ok(corners_near_player_edge[rand::thread_rng().gen_range(0..1)])
      }
      CurrentPath::Center(BotCenterPaths::PlayerPlacedCorner) => {
        center_edge_check_placed_corner_then_edge(self, gameboard)
      }
      CurrentPath::Center(BotCenterPaths::PlayerPlacedEdge) => self.auto_play(gameboard),
      _ => Err("Unknown Center Path".to_string()),
    }
  }

  pub fn auto_play(&self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    if let Some(coords) =
      gameboard.check_if_two_in_series(self.most_recent_chosen_coords.as_ref().unwrap())
    {
      Ok(coords)
    } else if let Some(coords) = gameboard.check_if_two_in_series(&gameboard.last_modified_tile) {
      Ok(coords)
    } else if let Some(coords) = gameboard.get_random_empty_corner_then_edge() {
      Ok(coords)
    } else {
      Err("No possible tile to place on".to_string())
    }
  }
}

fn center_corner_check_placed_edge(
  bot: &mut Bot,
  gameboard: &BoardConfig,
) -> Result<Coordinates, String> {
  bot.path = CurrentPath::DoubleWinCondition;

  let opposite_of_last_placed = bot
    .most_recent_chosen_coords
    .as_ref()
    .unwrap()
    .get_opposite_coordinates(&CENTER_TILE);

  if gameboard.get_board_state(&opposite_of_last_placed) != &BoardStates::Empty {
    bot
      .most_recent_chosen_coords
      .as_ref()
      .unwrap()
      .get_edges_around_corner(gameboard)
      .iter()
      .find_map(|coords| {
        if gameboard.get_board_state(coords) == &BoardStates::Empty {
          Some(*coords)
        } else {
          None
        }
      })
      .ok_or_else(|| "No open edge around 'most_recent_chosen_coords'".to_string())
  } else {
    bot.auto_play(gameboard)
  }
}

fn center_edge_check_placed_edge_near(
  bot: &Bot,
  gameboard: &BoardConfig,
) -> Result<Coordinates, String> {
  bot
    .most_recent_chosen_coords
    .as_ref()
    .unwrap()
    .get_edges_around_corner(gameboard)
    .iter()
    .find_map(|coords| {
      if gameboard.get_board_state(coords) == &BoardStates::Empty {
        Some(*coords)
      } else {
        None
      }
    })
    .ok_or_else(|| "No open edge around bot corner".to_string())
}

fn center_edge_check_placed_corner_then_edge(
  bot: &mut Bot,
  gameboard: &BoardConfig,
) -> Result<Coordinates, String> {
  bot.path = CurrentPath::DoubleWinCondition;

  let coords_around_player_edge = gameboard
    .last_modified_tile
    .get_corners_around_edge(gameboard);

  if coords_around_player_edge.iter().find_map(|coords| {
    if gameboard.get_board_state(coords) != &BoardStates::Empty {
      Some(gameboard.get_board_state(coords))
    } else {
      None
    }
  }) != Some(&bot.bot_symbol)
  {
    // this is when edge is far from bot corner
    bot.auto_play(gameboard)
  } else {
    center_edge_check_placed_edge_near(bot, gameboard)
  }
}

fn not_center_corner_check_placed_edge_near(
  bot: &mut Bot,
  gameboard: &BoardConfig,
) -> Result<Coordinates, String> {
  bot.path = CurrentPath::DoubleWinCondition;

  if gameboard
    .last_modified_tile
    .get_corners_around_edge(gameboard)
    .iter()
    .filter(|coords| {
      gameboard.get_board_state(coords) == gameboard.get_board_state(&gameboard.last_modified_tile)
    })
    .count()
    != 0
  {
    Ok(
      gameboard
        .last_modified_tile
        .get_opposite_coordinates(&CENTER_TILE),
    )
  } else {
    bot.path = CurrentPath::FocusDraw;
    bot.auto_play(gameboard)
  }
}

fn center_position_checks(bot: &mut Bot, gameboard: &BoardConfig) {
  match gameboard.get_board_position(&gameboard.last_modified_tile) {
    BoardPositions::Corner => {
      bot.most_recent_chosen_coords = bot.center_corner_checks(gameboard);
    }
    BoardPositions::Edge => {
      bot.most_recent_chosen_coords = bot.center_edge_checks(gameboard);
    }
    _ => bot.most_recent_chosen_coords = Err("Unknown board position".to_string()),
  }
}

fn not_center_position_checks(bot: &mut Bot, gameboard: &BoardConfig) {
  match gameboard.get_board_position(&gameboard.last_modified_tile) {
    BoardPositions::Corner => {
      bot.most_recent_chosen_coords = bot.not_center_corner_checks(gameboard);
    }
    BoardPositions::Edge => {
      bot.most_recent_chosen_coords = bot.not_center_edge_checks(gameboard);
    }
    _ => bot.most_recent_chosen_coords = Err("Unknown board position".to_string()),
  }
}
