use crate::coordinate_methods::*;
use crate::gameboard::*;

const CENTER_TILE: Coordinates = (1, 1);

// -- bugs --
//
// when player doesn't block the bot's forced corner placement under path
// CurrentPath::Center(BotCenterPaths::PlayerPlacedEdge)
// the bot doesn't take that and proceeds to follow through with a, funnily enough
// triple win condition

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
  PlayerPlacedEdge,         // move 2
  PlayerPlacedCorner,       // move 4
  PlayerPlacedEdgeThenEdge, // move 6
  Unknown,
}

#[derive(PartialEq, Clone, Debug)]
pub enum PlayerCenterPaths {
  PlayerDidntPlaceCenter, // ???????
  PlayerPlacedEdgeNear,   // move 3
  Unknown,
}

impl Bot {
  pub fn new() -> Self {
    Bot {
      path: CurrentPath::Unknown,
      bot_symbol: BoardStates::Empty,
      most_recent_chosen_coords: Err("No error has been given".to_string()),
    }
  }

  // --FIRST TWO MOVES--
  // if you get center and player places on an edge, win is Guaranteed
  // if you get center and player places on corner, win is possible
  // if you don't get center, never place on edge, always corner

  // --PLAYER PLACES ON EDGE WITH YOU CENTER--
  // place a piece on a corner next to player to force them to block your first 3 in a row
  // place on edge next to corner you placed
  // win

  // --PLAYER PLACES ON CORNER WITH YOU CENTER--
  // place on corner opposite of player
  // if player places on an edge win is Guaranteed
  // if player places on any corner, FocusDraw

  // --PLAYER HAS CENTER--
  // win is Medium
  // place on corner
  // if player places on opposite corner place on a corner and FocusDraw
  // if player places on edge next to your piece get opposite and place there
  //   - if from there player places on the corner over their edge from your corner,
  //   win is Guaranteed
  //   - otherwise FocusDraw
  // if player places anywhere else FocusDraw
  //
  //
  // first 2 moves, check the center and update status

  pub fn choose_coordinates(&mut self, gameboard: &BoardConfig) {
    match &self.path {
      CurrentPath::Center(_) => {
        //
        match gameboard.get_board_position(&gameboard.last_modified_tile) {
          BoardPositions::Corner => {
            self.most_recent_chosen_coords = self.center_corner_checks(gameboard);
          }
          BoardPositions::Edge => {
            self.most_recent_chosen_coords = self.center_edge_checks(gameboard);
          }
          _ => self.most_recent_chosen_coords = Err("Unknown board position".to_string()),
        }
      }

      CurrentPath::NotCenter(_) => {
        match gameboard.get_board_position(&gameboard.last_modified_tile) {
          BoardPositions::Corner => {
            self.most_recent_chosen_coords = self.not_center_corner_checks(gameboard);
          }
          BoardPositions::Edge => {
            self.most_recent_chosen_coords = self.not_center_edge_checks(gameboard);
          }
          _ => self.most_recent_chosen_coords = Err("Unknown board position".to_string()),
        }
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

  // this should only be called within the first 2 moves
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

  // this should only be called within the first 2 moves
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
        //=========================================================================
        //=========IT'S POSSIBLE THIS WON'T WORK AND NEEDS TO BE LOOKED AT=========
        //=========================================================================
        self.path = CurrentPath::DoubleWinCondition;
        //========

        if gameboard
          .last_modified_tile
          .get_coords_around_excluding_center()
          .iter()
          .filter(|coords| {
            gameboard.get_board_state(coords)
              == gameboard.get_board_state(&gameboard.last_modified_tile)
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
          self.path = CurrentPath::FocusDraw;
          self.auto_play(gameboard)
        }
      }
      _ => Err("Unknown NotCenter Bot Path".to_string()),
    }
  }

  pub fn not_center_edge_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    let edge_coords_around_bot_corner = self
      .most_recent_chosen_coords
      .as_ref()
      .unwrap()
      .get_coords_around_excluding_center()
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
        self.path = CurrentPath::DoubleWinCondition;

        let opposite_of_last_placed = self
          .most_recent_chosen_coords
          .as_ref()
          .unwrap()
          .get_opposite_coordinates(&CENTER_TILE);

        if gameboard.get_board_state(&opposite_of_last_placed) == &BoardStates::Empty {
          self
            .most_recent_chosen_coords
            .as_ref()
            .unwrap()
            .get_coords_around_excluding_center()
            .iter()
            .find_map(|coords| {
              if gameboard.get_board_state(coords) == &BoardStates::Empty {
                Some(*coords)
              } else {
                None
              }
            })
            .ok_or_else(|| "No open edge around 'chosen_placement'".to_string())
        } else {
          self.auto_play(gameboard)
        }
      }
      _ => Err("Unknown Center Path".to_string()),
    }
  }

  // === maybe split up the paths into their own functions ===
  pub fn center_edge_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    match &self.path {
      CurrentPath::Center(BotCenterPaths::Unknown) => {
        self.path = CurrentPath::Center(BotCenterPaths::PlayerPlacedEdge);

        let corners_near_player_edge = gameboard
          .last_modified_tile
          .get_coords_around_excluding_center();

        // maybe declare some sort of global RNG string and get a random
        // one of the two corners
        Ok(corners_near_player_edge[0])
      }
      CurrentPath::Center(BotCenterPaths::PlayerPlacedCorner) => {
        self.path = CurrentPath::DoubleWinCondition;

        let coords_around_player_edge = gameboard
          .last_modified_tile
          .get_coords_around_excluding_center();

        if coords_around_player_edge
          .iter()
          .find_map(|coords| {
            if gameboard.get_board_state(coords) != &BoardStates::Empty {
              Some(gameboard.get_board_state(coords))
            } else {
              None
            }
          })
          .unwrap()
          != &self.bot_symbol
        {
          // this is when edge is far from bot corner
          //
          // unknown as to if this will cause problems so i'll keep it here
          //
          // coords_around_player_edge
          // .iter()
          // .find_map(|coords| {
          // if gameboard.get_board_state(coords) != &BoardStates::Empty {
          // Some(*coords)
          // } else {
          // None
          // }
          // })
          // .ok_or_else(|| "No available corners around player's edge".to_string())
          self.auto_play(gameboard)
        } else {
          // this is when edge is near bot corner
          self
            .most_recent_chosen_coords
            .as_ref()
            .unwrap()
            .get_coords_around_excluding_center()
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
    } else if let Some(coords) = gameboard.get_random_empty_tile() {
      Ok(coords)
    } else {
      Err("No possible tile to place on".to_string())
    }
  }
}
