use std::env::args;
use tictactoe_with_ai::gameplay::{run_args, run_gameplay};

fn main() {
  println!("\n\n");
  println!(" -- run the program with 'bot_play' or 'free_play' for other modes -- ");
  println!("\n\n");

  let args: Vec<String> = args().collect();

  if args.len() > 1 {
    if let Err(error) = run_args(args) {
      eprintln!("An error has occured: '{}'", error);
    }

    return;
  }

  if let Err(error) = run_gameplay() {
    eprintln!("An error has occured: '{}'", error);
  }
}
