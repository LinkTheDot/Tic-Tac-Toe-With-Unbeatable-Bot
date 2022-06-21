use crate::coordinate_methods::*;
use crate::gameboard::*;
use crate::gameplay::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Bot {
  pub win_mode: WinChances,
  pub personal_board: BoardConfig,
  pub player_has_center: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub enum WinChances {
  Guaranteed,
  High,
  Medium,
  FocusDraw,
}

pub enum CurrentPath {
  Center(CenterPaths),
  NotCenter(NotCenterPaths),
}

pub enum CenterPaths {
  PlayerPlacedEdge,
  PlayerPlacedCorner,
  PlayerPlacedCornerThenEdge,
  PlayerPlacedCornerThenCorner,
  Unknown,
}

pub enum NotCenterPaths {
  PlayerDidntPlaceCenter, // ???????
  PlayerPlacedOppositeCorner,
  PlayerPlacedEdgeFar,
  PlayerPlacedEdgeNear,
  Unknown,
}

impl Bot {
  pub fn new() -> Self {
    let win_mode = WinChances::High;
    let personal_board = BoardConfig::new();

    Bot {
      win_mode,
      personal_board,
      player_has_center: false,
    }
  }

  // --FIRST TWO MOVES--
  // if you get center and player places on an edge, win is Guaranteed
  // if you get center and player places on corner, win is possible
  // if you don't get center, never place on edge, always corner

  // --PLAYER PLACES ON EDGE WITH YOU CENTER--
  // place a piece on a corner to force them to block your first 3 in a row
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

  pub fn bot_actions(&mut self, game_board: BoardConfig) -> Coordinates {
    //self.win_mode = self.evaluate_win_chances(game_board);

    (0, 0)
  }

  //pub fn evaluate_win_chances(&mut self, game_board: BoardConfig) -> WinChances {
  //  if game_board.tiles_covered <= 1 // /
  //    && game_board.get_board_state(&(1, 1)) == &BoardStates::Empty
  //  {
  //    WinChances::High
  //  } else if game_board.get_board_state(&(1, 1)) != &BoardStates::Empty {
  //    self.player_has_center = true;
  //    WinChances::Medium
  //  } else {
  //    WinChances::Medium
  //  }
  //}
}
