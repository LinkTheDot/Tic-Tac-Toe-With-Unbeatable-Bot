use crate::coordinate_methods::*;
use crate::gameboard::*;
use crate::gameplay::*;
use rand::prelude::*;
use std::error::Error;

#[derive(PartialEq, Debug)]
pub struct Bot {
  pub path: CurrentPath,
  pub bot_symbol: BoardStates,
  pub chosen_placement: Result<Coordinates, String>,
  pub last_placed_tile: Result<Coordinates, String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CurrentPath {
  Center(BotCenterPaths),
  NotCenter(PlayerCenterPaths),
  FocusDraw,
  Unknown,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BotCenterPaths {
  PlayerPlacedEdge,             // move 2
  PlayerPlacedCorner,           // move 4
  PlayerPlacedCornerThenEdge,   // move 6
  PlayerPlacedCornerThenCorner, // move 6
  Unknown,
}

#[derive(PartialEq, Clone, Debug)]
pub enum PlayerCenterPaths {
  PlayerDidntPlaceCenter,     // ???????
  PlayerPlacedOppositeCorner, // move 3
  PlayerPlacedEdgeFar,        // move 3
  PlayerPlacedEdgeNear,       // move 3
  PlayerPlacedCornerNearEdge, // move 5
  Unknown,
}

impl Bot {
  pub fn new() -> Self {
    Bot {
      path: CurrentPath::Unknown,
      bot_symbol: BoardStates::Empty,
      chosen_placement: Err("No error has been given".to_string()),
      last_placed_tile: Err("No error has been given".to_string()),
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

  pub fn bot_actions(&mut self, gameboard: &BoardConfig) -> Result<&Coordinates, &String> {
    self.chosen_placement = Err("no input initiated".to_string());

    match &self.path {
      CurrentPath::Center(_) => {
        // /
        match gameboard.get_board_position(&gameboard.last_modified_tile) {
          BoardPositions::Corner => {
            self.chosen_placement = self.center_corner_checks(&gameboard);
          }
          BoardPositions::Edge => {
            self.chosen_placement = self.center_edge_checks(&gameboard);
          }
          _ => self.chosen_placement = Err("Unknown board position".to_string()),
        }
      }
      CurrentPath::NotCenter(_) => {
        match gameboard.get_board_position(&gameboard.last_modified_tile) {
          BoardPositions::Corner => {
            self.chosen_placement = self.not_center_corner_checks(&gameboard)
          }
          BoardPositions::Edge => {
            self.chosen_placement = self.not_center_edge_checks(&gameboard);
          }
          _ => self.chosen_placement = Err("Unknown board position".to_string()),
        }
      }
      CurrentPath::FocusDraw => {
        // bunch of code shit to make sure you never lose
        // but also check if there's any opportunity to win
        // because the player is stupid
        //
        // *obviously most of it will be split into alread existing
        // pre-planned methods for coordinates and the bot
      }
      CurrentPath::Unknown => {
        self.path = self.check_if_center_or_not(&gameboard);

        if self.path == CurrentPath::Center(BotCenterPaths::Unknown) {
          self.chosen_placement = Ok((1, 1));
          self.last_placed_tile = self.chosen_placement.clone();
        } else {
          self.chosen_placement = self.initial_check_of_player_center_paths(gameboard);
          self.last_placed_tile = self.chosen_placement.clone();
        }
      }
    }

    self.chosen_placement.as_ref()
  }

  // this should only be called within the first 2 moves
  pub fn check_if_center_or_not(&self, gameboard: &BoardConfig) -> CurrentPath {
    if gameboard.tiles_covered == 0 // /
      && gameboard.get_board_state(&(1, 1)) == &BoardStates::Empty
    {
      CurrentPath::Center(BotCenterPaths::Unknown)
    } else if gameboard.get_board_state(&(1, 1)) == &BoardStates::Empty {
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

          Ok((1, 1))
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
        if let Ok(bot_tile) = &self.last_placed_tile {
          let opposite_corner_state =
            gameboard.get_board_state(&bot_tile.get_opposite_coordinates(&(1, 1)));

          if opposite_corner_state != &BoardStates::Empty {
            self.path = CurrentPath::FocusDraw;
            self.path = CurrentPath::NotCenter(PlayerCenterPaths::PlayerPlacedOppositeCorner);

            Ok(gameboard.get_random_empty_corner().unwrap())
          } else {
            self.path = CurrentPath::FocusDraw;
            self.block_player_win(&gameboard)
          }
        } else {
          Err("Last placed tile is empty".to_string())
        }
      }
      CurrentPath::NotCenter(PlayerCenterPaths::PlayerPlacedEdgeNear) => {
        if gameboard
          .last_modified_tile
          .get_coords_around_excluding_center()
          .iter()
          .filter(|coords| {
            gameboard.get_board_state(&coords)
              == gameboard.get_board_state(&gameboard.last_modified_tile)
          })
          .count()
          != 0
        {
          Ok(
            gameboard
              .last_modified_tile
              .get_opposite_coordinates(&(1, 1)),
          )
        } else {
          self.path = CurrentPath::FocusDraw;
          self.block_player_win(gameboard)
        }
      }
      _ => Err("Unknown board position".to_string()),
    }
  }

  pub fn not_center_edge_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    let edge_coords_around_bot_corner = self
      .last_placed_tile
      .as_ref()
      .unwrap()
      .get_coords_around_excluding_center()
      .into_iter()
      .filter(|edge| edge == &gameboard.last_modified_tile)
      .collect::<Vec<Coordinates>>();

    if edge_coords_around_bot_corner.len() != 0 {
      self.path = CurrentPath::NotCenter(PlayerCenterPaths::PlayerPlacedEdgeNear);

      Ok(edge_coords_around_bot_corner[0].get_opposite_coordinates(&(1, 1)))
    } else {
      self.path = CurrentPath::FocusDraw;

      self.block_player_win(gameboard)
    }
  }

  pub fn center_corner_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    todo!()
  }

  pub fn center_edge_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    todo!()
  }

  pub fn block_player_win(&self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    // take the last input from the player and see if there's a 2 in a row,
    // if so place opposite from that
    //
    // also check if anything across from theirs is a match

    match gameboard.get_board_state(&gameboard.last_modified_tile) {
      _ => Err("".to_string()),
    }
  }
}
