use std::env::args;
use tictactoe_with_ai::gameplay::{check_args_for_gamemodes, run_gameplay, GameConfig};

fn main() {
  println!("\n\n -- run the program with 'bot_play' or 'free_play' for other modes -- \n\n");

  let args: Option<String> = args().nth(1);
  let mut gameconfig = GameConfig::new()
    .unwrap_or_else(|error| panic!("An error has occured while grabbing config: '{error}'"));

  if let Some(gamemode) = args {
    if let Err(error) = check_args_for_gamemodes(gamemode, &mut gameconfig) {
      eprintln!("An error has occured processing args: '{}'", error);
    }
  } else if let Err(error) = run_gameplay(&mut gameconfig) {
    eprintln!("An error has occured during gameplay: '{}'", error);
  }
}
