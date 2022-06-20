#![allow(unused)]
pub mod bot;
pub mod coordinate_methods;
pub mod default_impls;
pub mod gameboard;
pub mod gameplay;

use crate::bot::*;
use crate::gameboard::*;
use crate::gameplay::*;
use coordinate_methods::*;

fn main() {
  if let Err(error) = run_gameplay() {
    eprintln!("An error has occured: '{}'", error);
  }
}
