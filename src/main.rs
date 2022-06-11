#![allow(unused)]
pub mod bot;
pub mod gameboard;
pub mod gameplay;

use crate::bot::*;
use crate::gameboard::*;
use crate::gameplay::*;

fn main() {
  if let Err(error) = run_gameplay() {
    println!("An error has occured: {}", error);
  }
}
