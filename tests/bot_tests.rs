use tictactoe_with_ai::bot::*;
use tictactoe_with_ai::gameboard::*;

#[cfg(test)]
mod not_center_logic {
  use super::*;

  #[test]
  #[ignore]
  pub fn edge_checks() {
    let mut gameboard = BoardConfig::new();
    let mut bot = Bot::new();

    // where X is player
    // where O is bot

    gameboard.place_tile((1, 1), BoardStates::X);

    bot.path = bot.check_if_center_or_not(&gameboard);
    bot.chosen_placement = bot.initial_check_of_player_center_paths(&gameboard);

    gameboard.place_tile(
      match bot.chosen_placement.as_ref() {
        Ok(coords) => *coords,
        Err(err) => panic!("An error has occurred '{}'", err),
      },
      BoardStates::O,
    );

    gameboard.place_tile(gameboard.get_random_empty_edge().unwrap(), BoardStates::X);

    bot.chosen_placement = bot.not_center_edge_checks(&gameboard);

    gameboard.place_tile(
      match bot.chosen_placement.as_ref() {
        Ok(coords) => *coords,
        Err(err) => panic!("An error has occurred '{}'", err),
      },
      BoardStates::O,
    );

    gameboard.print_board();
  }

  #[test]
  #[ignore]
  pub fn corner_checks() {
    let mut gameboard = BoardConfig::new();
    let mut bot = Bot::new();

    // where X is player
    // where O is bot

    gameboard.place_tile((1, 1), BoardStates::X);

    bot.path = bot.check_if_center_or_not(&gameboard);
    bot.chosen_placement = bot.initial_check_of_player_center_paths(&gameboard);

    gameboard.place_tile(
      match bot.chosen_placement.as_ref() {
        Ok(coords) => *coords,
        Err(err) => panic!("An error has occurred '{}'", err),
      },
      BoardStates::O,
    );

    gameboard.place_tile(gameboard.get_random_empty_corner().unwrap(), BoardStates::X);

    bot.chosen_placement = bot.not_center_corner_checks(&gameboard);

    gameboard.place_tile(
      match bot.chosen_placement.as_ref() {
        Ok(coords) => *coords,
        Err(err) => panic!("An error has occurred '{}'", err),
      },
      BoardStates::O,
    );

    gameboard.print_board();
  }
}
