use crate::bot::Bot;
use crate::gameboard::BoardConfig;
use crate::gameplay::GameConfig;

impl Default for GameConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for BoardConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for Bot {
  fn default() -> Self {
    Self::new()
  }
}
