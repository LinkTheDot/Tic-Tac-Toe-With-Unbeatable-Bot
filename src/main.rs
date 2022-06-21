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
