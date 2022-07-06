use tictactoe_with_ai::bot::*;
use tictactoe_with_ai::gameboard::*;

#[ignore]
pub fn not_center_checks_logic_works() {
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
      Err(err) => panic!("err '{}'", err),
    },
    BoardStates::O,
  );

  //player places on any of the 3 open corners
  {
    gameboard.place_tile(gameboard.get_random_empty_corner().unwrap(), BoardStates::X);

    bot.chosen_placement = bot.not_center_corner_checks(&gameboard);

    gameboard.place_tile(
      match bot.chosen_placement.as_ref() {
        Ok(coords) => *coords,
        Err(err) => panic!("err '{}'", err),
      },
      BoardStates::O,
    );

    gameboard.print_board();
  }

  //player places on any of the 3 open edges
  {
    gameboard.place_tile(gameboard.get_random_empty_edge().unwrap(), BoardStates::X);

    bot.chosen_placement = bot.not_center_edge_checks(&gameboard);

    gameboard.place_tile(
      match bot.chosen_placement.as_ref() {
        Ok(coords) => *coords,
        Err(err) => panic!("err '{}'", err),
      },
      BoardStates::O,
    );

    gameboard.print_board();
  }
}
