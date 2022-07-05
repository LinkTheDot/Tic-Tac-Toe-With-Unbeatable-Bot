#![allow(unused)]
pub mod bot;
pub mod coordinate_methods;
pub mod defaults;
pub mod gameboard;
pub mod gameplay;
mod tests;

use crate::bot::*;
use crate::gameboard::*;
use crate::gameplay::*;
use coordinate_methods::*;

fn main() {
  if let Err(error) = run_gameplay() {
    eprintln!("An error has occured: '{}'", error);
  }
}

// todo list -
// gameboard {
//  check_if_2_in_series()
// }
// bot {
//  center_corner_checks()
//  center_edge_checks()
//  block_player_win()
// }
