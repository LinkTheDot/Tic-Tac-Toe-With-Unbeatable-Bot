use tictactoe_with_ai::gameplay::*;

fn main() {
  if let Err(error) = run_gameplay() {
    eprintln!("An error has occured: '{}'", error);
  }
}

// todo list -
// bot {
//  center_corner_checks()
//  center_edge_checks()
//  block_player_win()
// }
