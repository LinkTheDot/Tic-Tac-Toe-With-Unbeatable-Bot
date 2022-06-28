#![allow(unused)]
pub mod bot;
pub mod coordinate_methods;
pub mod defaults;
pub mod gameboard;
pub mod gameplay;
pub mod tests;

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
//  get_random_empty_corner()
//  get_random_empty_edge()
// }
//
// coordinate_methods {
//  get_edges_around_corner()
//  get_corners_around_edge()
//  get_all_corner_states()
//  get_all_edge_states()
//  check_if_win_is_possible()
// }
//
// bot {
//  not_center_edge_checks()
//  center_corner_checks()
//  center_edge_checks()
//  block_player_win()
// }
