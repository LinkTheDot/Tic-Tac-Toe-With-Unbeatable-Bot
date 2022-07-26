use std::env::args;
use tictactoe_with_ai::gameplay::{check_args_for_gamemodes, run_gameplay, GameConfig};

fn main() {
  let possible_gamemode_arguments = args().nth(1);

  let mut gameconfig = GameConfig::new()
    .unwrap_or_else(|error| panic!("An error has occured while grabbing config: '{error}'"));

  if let Some(gamemode) = possible_gamemode_arguments {
    check_args_for_gamemodes(gamemode, &mut gameconfig)
      .unwrap_or_else(|error| eprintln!("An error has occured processing args: '{error}'"));
  } else if let Err(error) = run_gameplay(&mut gameconfig) {
    eprintln!("An error has occured during gameplay: '{error}'");
  }
}
