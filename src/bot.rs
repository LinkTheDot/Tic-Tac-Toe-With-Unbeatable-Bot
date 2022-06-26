use crate::coordinate_methods::*;
use crate::gameboard::*;
use crate::gameplay::*;
use std::error::Error;

#[derive(PartialEq, Clone, Debug)]
pub struct Bot {
  pub win_mode: WinChances,
  pub personal_board: BoardConfig,
  pub player_has_center: bool,
  pub path: CurrentPath,
  pub bot_symbol: BoardStates,
  pub chosen_placement: Result<Coordinates, String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum WinChances {
  Guaranteed,
  High,
  Medium,
  FocusDraw,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CurrentPath {
  Center(BotCenterPaths),
  NotCenter(PlayerCenterPaths),
  Unknown,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BotCenterPaths {
  PlayerPlacedEdge,
  PlayerPlacedCorner,
  PlayerPlacedCornerThenEdge,
  PlayerPlacedCornerThenCorner,
  Unknown,
}

#[derive(PartialEq, Clone, Debug)]
pub enum PlayerCenterPaths {
  PlayerDidntPlaceCenter, // ???????
  PlayerPlacedOppositeCorner,
  PlayerPlacedEdgeFar,
  PlayerPlacedEdgeNear,
  Unknown,
}

impl Bot {
  pub fn new() -> Self {
    Bot {
      win_mode: WinChances::High,
      personal_board: BoardConfig::new(),
      player_has_center: false,
      path: CurrentPath::Unknown,
      bot_symbol: BoardStates::Empty,
      chosen_placement: Err("template".to_string()),
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
  //   - if from there player places on the corner across from yours, win is Guaranteed
  //   - otherwise FocusDraw
  // if player places anywhere else FocusDraw
  //
  //
  // first 2 moves, check the center and update status

  pub fn bot_actions(&mut self, gameboard: &BoardConfig) -> Result<&Coordinates, &String> {
    self.chosen_placement = Err("no input initiated".to_string());

    let _x = match &self.path {
      CurrentPath::Center(x) => (),
      CurrentPath::NotCenter(x) => (),
      // since this will only ever be Unknown once this will determine the state
      // of the Center tile
      CurrentPath::Unknown => {
        self.path = self.check_center_path(gameboard);

        if self.path == CurrentPath::Center(BotCenterPaths::Unknown) {
          self.chosen_placement = Ok((1, 1));
        } else {
          // need to check if it's PlayerCenterPath is unknown or didn't place center
          // do 2 different things with those
          // make separate function? if unknown place center else do other thing?
          self.chosen_placement = Err("a".to_string()); // not finished
        }
      }
    };

    self.chosen_placement.as_ref()
  }

  pub fn check_center_path(&self, gameboard: &BoardConfig) -> CurrentPath {
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

  pub fn check_player_center_paths(&self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    if let CurrentPath::NotCenter(path_state) = &self.path {
      match path_state {
        PlayerCenterPaths::PlayerDidntPlaceCenter => Ok((1, 1)),
        PlayerCenterPaths::Unknown => {
          // run some checks and determine where to place,
          // also switch to win chance FocusDraw
          Ok((0, 0))
        }
        _ => Err("neither 'Unknown' or 'DidntPlaceCenter'".to_string()),
      }
    } else {
      Err("non-NotCenter path has been called".to_string())
    }
  }
}
