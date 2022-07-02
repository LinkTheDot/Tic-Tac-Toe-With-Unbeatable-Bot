#[cfg(test)]
use crate::bot::*;
use crate::coordinate_methods::*;
use crate::gameboard::*;
use crate::gameplay::*;

#[test]
pub fn not_center_checks_logic_works() {
  let mut gameboard = BoardConfig::new();
  let mut bot = Bot::new();

  // where X is player
  // where O is bot

  gameboard.place_tile((1, 1), BoardStates::X);

  bot.path = bot.check_if_center_or_not(&gameboard);
  bot.chosen_placement = bot.initial_check_of_player_center_paths(&gameboard);

  gameboard.place_tile(
    *bot.chosen_placement.as_ref().unwrap_or_else(|err| {
      eprintln!("An error has occurred '{}'", err);
      &(10, 10)
    }),
    BoardStates::O,
  );

  //player places on any of the 3 open corners
  {
    gameboard.place_tile(gameboard.get_random_empty_corner().unwrap(), BoardStates::X);

    bot.chosen_placement = bot.not_center_corner_checks(&gameboard);

    gameboard.place_tile(
      *bot.chosen_placement.as_ref().unwrap_or_else(|err| {
        eprintln!("An error has occurred '{}'", err);
        &(10, 10)
      }),
      BoardStates::O,
    );

    gameboard.print_board();
  }

  //player places on any of the 3 open edges
  {
    gameboard.place_tile(gameboard.get_random_empty_edge().unwrap(), BoardStates::X);

    bot.chosen_placement = bot.not_center_edge_checks(&gameboard);

    gameboard.place_tile(
      *bot.chosen_placement.as_ref().unwrap_or_else(|err| {
        eprintln!("An error has occurred '{}'", err);
        &(10, 10)
      }),
      BoardStates::O,
    );

    gameboard.print_board();
  }
}
