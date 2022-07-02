#[cfg(test)]
use crate::gameboard::*;
use crate::gameplay::*;

#[test]
fn testing_coordinate_conversion() {
  let num = 5;

  assert_eq!(((num - 1) / 3, (num - 1) % 3), (1, 1))
}

#[test]
fn check_if_win_logic_works() {
  let mut game_config = GameConfig::new();
  let latest_tile_true = (0, 0);
  let latest_tile_false = (1, 0); // edge that's same but not in row
  let latest_tile_center = (1, 1);

  //for latest_tile_true
  game_config.game_board.tiles[0][0].board_state = BoardStates::X;
  game_config.game_board.tiles[0][1].board_state = BoardStates::X;
  game_config.game_board.tiles[0][2].board_state = BoardStates::X;

  //for latest_tile_false
  game_config.game_board.tiles[1][0].board_state = BoardStates::X;

  //center to edge/corner
  game_config.game_board.tiles[1][1].board_state = BoardStates::O;
  game_config.game_board.tiles[2][0].board_state = BoardStates::O;
  game_config.game_board.tiles[2][1].board_state = BoardStates::O;

  println!(
    "{:?}",
    game_config
      .game_board
      .get_board_position(&latest_tile_center)
  );

  game_config.game_board.last_modified_tile = latest_tile_true;
  assert_eq!(game_config.check_if_win(), true);
  game_config.game_board.last_modified_tile = latest_tile_false;
  assert_eq!(game_config.check_if_win(), false);
  game_config.game_board.last_modified_tile = latest_tile_center;
  assert_eq!(game_config.check_if_win(), false);
}
