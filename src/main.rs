use std::env::args;
use tictactoe_with_ai::gameplay::{run_gamemode, GameConfig};

fn main() {
  let possible_gamemode_arguments = args().nth(1);

  let gameconfig = GameConfig::new()
    .unwrap_or_else(|error| panic!("An error has occured while grabbing config: '{error}'"));

  if let Err(error) = run_gamemode(possible_gamemode_arguments, gameconfig) {
    eprintln!("An error has occured: '{error}'");
  }
}
