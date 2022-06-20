use crate::coordinate_methods::*;
use crate::gameboard::*;
use crate::gameplay::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Bot {
  pub win_mode: WinChances,
  pub personal_board: BoardConfig,
}

#[derive(PartialEq, Clone, Debug)]
pub enum WinChances {
  High,
  Medium,
  FocusDraw,
}

impl Bot {
  pub fn new() -> Self {
    let win_mode = WinChances::High;
    let personal_board = BoardConfig::new();

    Bot {
      win_mode,
      personal_board,
    }
  }
  pub fn evaluate_win_chances(&self, game_board: BoardConfig) -> WinChances {
    if game_board.tiles_covered <= 1 // /
      && game_board.get_board_state(&(1, 1)) == &BoardStates::Empty
    {
      WinChances::High
    } else {
      WinChances::Medium
    }
  }
}
