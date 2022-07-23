use tictactoe_with_ai::coordinate_methods::GRID_SIZE;
use tictactoe_with_ai::gameboard::*;
use tictactoe_with_ai::gameplay::*;

#[test]
fn testing_coordinate_conversion() {
  let index_at = 5;
  let coordinate_conversion = ((index_at - 1) / GRID_SIZE, (index_at - 1) % GRID_SIZE);
  let expected_outcome = (1, 1);

  assert_eq!(coordinate_conversion, expected_outcome);
}

#[cfg(test)]
mod check_if_win_logic {
  use super::*;

  #[test]
  fn side_vaild_win() {
    let mut gameconfig = GameConfig::new()
      .unwrap_or_else(|error| panic!("An error has occured while grabbing config: '{error}'"));
    let checking_from = (0, 0);
    let expected_outcome = true;

    gameconfig.gameboard.last_modified_tile = Some(checking_from);

    //X|X|X
    //-|-|-
    //-|-|-
    gameconfig.gameboard.place_tile(&(0, 0), &BoardStates::X);
    gameconfig.gameboard.place_tile(&(0, 1), &BoardStates::X);
    gameconfig.gameboard.place_tile(&(0, 2), &BoardStates::X);

    assert_eq!(gameconfig.check_if_win(), expected_outcome);
  }

  #[test]
  fn in_between_valid_win() {
    let mut gameconfig = GameConfig::new()
      .unwrap_or_else(|error| panic!("An error has occured while grabbing config: '{error}'"));
    let checking_from = (1, 0);
    let expected_outcome = true;

    gameconfig.gameboard.last_modified_tile = Some(checking_from);

    //X|-|-
    //X|-|-
    //X|-|-
    gameconfig.gameboard.place_tile(&(0, 0), &BoardStates::X);
    gameconfig.gameboard.place_tile(&(1, 0), &BoardStates::X);
    gameconfig.gameboard.place_tile(&(2, 0), &BoardStates::X);

    assert_eq!(gameconfig.check_if_win(), expected_outcome);
  }

  #[test]
  fn two_in_a_row_possible_overflow() {
    let mut gameconfig = GameConfig::new()
      .unwrap_or_else(|error| panic!("An error has occured while grabbing config: '{error}'"));
    let checking_from = (1, 0);
    let expected_outcome = false;

    gameconfig.gameboard.last_modified_tile = Some(checking_from);

    //X|-|-
    //X|-|-
    //-|-|-
    gameconfig.gameboard.place_tile(&(0, 0), &BoardStates::X);
    gameconfig.gameboard.place_tile(&(1, 0), &BoardStates::X);

    assert_eq!(gameconfig.check_if_win(), expected_outcome);
  }
}
