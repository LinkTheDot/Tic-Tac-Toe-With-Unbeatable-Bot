use crate::coordinate_methods::*;
use crate::gameboard::*;
use rand::prelude::*;

const CENTER_TILE: Coordinates = (1, 1);
const FIRST_MOVE: u8 = 0;

#[derive(PartialEq, Debug)]
pub struct Bot {
  pub path: CurrentPath,
  pub bot_symbol: BoardStates,
  pub most_recent_chosen_coords: Result<Coordinates, String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CurrentPath {
  Center(BotPaths),
  PlayerDidntPlaceCenter,
  DoubleWinCondition,
  FocusDraw,
  FirstMove,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BotPaths {
  PlayerPlacedEdge,
  PlayerPlacedCorner,
  Unknown,
}

impl Bot {
  pub fn new() -> Self {
    Bot {
      path: CurrentPath::FirstMove,
      bot_symbol: BoardStates::Empty,
      most_recent_chosen_coords: Err("No error has been given".to_string()),
    }
  }

  pub fn choose_coordinates(&mut self, gameboard: &BoardConfig) {
    match &self.path {
      CurrentPath::Center(_) => {
        center_position_checks(self, gameboard);
      }
      CurrentPath::FocusDraw | CurrentPath::DoubleWinCondition => {
        self.most_recent_chosen_coords = self.auto_play(gameboard);
      }
      CurrentPath::FirstMove => {
        self.path = self.check_if_center_is_available_or_not(gameboard);

        self.most_recent_chosen_coords = match self.path {
          CurrentPath::Center(BotPaths::Unknown) => Ok(CENTER_TILE),
          CurrentPath::PlayerDidntPlaceCenter => {
            self.path = CurrentPath::FocusDraw;
            Ok(CENTER_TILE)
          }
          _ => gameboard
            .get_random_empty_corner_then_edge()
            .ok_or_else(|| "Failed to get random tile".to_string()),
        };
      }
      _ => self.most_recent_chosen_coords = Err("Incorrect path input".to_string()),
    }
  }

  /// This will only be called in the first 2 moves
  pub fn check_if_center_is_available_or_not(&self, gameboard: &BoardConfig) -> CurrentPath {
    let center_board_state = gameboard.get_board_state(&CENTER_TILE);

    if gameboard.tiles_covered == FIRST_MOVE && center_board_state == &BoardStates::Empty {
      CurrentPath::Center(BotPaths::Unknown)
    } else if center_board_state == &BoardStates::Empty {
      CurrentPath::PlayerDidntPlaceCenter
    } else {
      CurrentPath::FocusDraw
    }
  }

  pub fn center_corner_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    match &self.path {
      CurrentPath::Center(BotPaths::Unknown) => {
        self.path = CurrentPath::Center(BotPaths::PlayerPlacedCorner);

        let opposite_coords = gameboard
          .last_modified_tile
          .unwrap()
          .get_opposite_coordinates(&CENTER_TILE);

        if gameboard.get_board_state(&opposite_coords) == &BoardStates::Empty {
          Ok(opposite_coords)
        } else {
          Err("Opposite corner is filled".to_string())
        }
      }
      CurrentPath::Center(BotPaths::PlayerPlacedCorner) => {
        self.path = CurrentPath::FocusDraw;

        self.auto_play(gameboard)
      }
      CurrentPath::Center(BotPaths::PlayerPlacedEdge) => {
        center_corner_check_placed_edge(self, gameboard)
      }
      _ => Err("Unknown Center Path".to_string()),
    }
  }

  pub fn center_edge_checks(&mut self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    match &self.path {
      CurrentPath::Center(BotPaths::Unknown) => {
        self.path = CurrentPath::Center(BotPaths::PlayerPlacedEdge);

        let corners_near_player_edge = gameboard
          .last_modified_tile
          .unwrap()
          .get_corners_around_edge(gameboard);

        Ok(corners_near_player_edge[rand::thread_rng().gen_range(0..1)])
      }
      CurrentPath::Center(BotPaths::PlayerPlacedCorner) => {
        center_edge_check_placed_corner_then_edge(self, gameboard)
      }
      CurrentPath::Center(BotPaths::PlayerPlacedEdge) => self.auto_play(gameboard),
      _ => Err("Unknown Center Path".to_string()),
    }
  }

  pub fn auto_play(&self, gameboard: &BoardConfig) -> Result<Coordinates, String> {
    if let Some(coords) =
      gameboard.check_if_two_in_series(self.most_recent_chosen_coords.as_ref().unwrap())
    {
      Ok(coords)
    } else if let Some(coords) =
      gameboard.check_if_two_in_series(&gameboard.last_modified_tile.unwrap())
    {
      Ok(coords)
    } else if let Some(coords) = gameboard.get_random_empty_corner_then_edge() {
      Ok(coords)
    } else {
      Err("No possible tile to place on".to_string())
    }
  }
}

fn center_corner_check_placed_edge(
  bot: &mut Bot,
  gameboard: &BoardConfig,
) -> Result<Coordinates, String> {
  bot.path = CurrentPath::DoubleWinCondition;

  let opposite_of_last_placed = bot
    .most_recent_chosen_coords
    .as_ref()
    .unwrap()
    .get_opposite_coordinates(&CENTER_TILE);

  if gameboard.get_board_state(&opposite_of_last_placed) != &BoardStates::Empty {
    bot
      .most_recent_chosen_coords
      .as_ref()
      .unwrap()
      .get_edges_around_corner(gameboard)
      .iter()
      .find_map(|coords| {
        if gameboard.get_board_state(coords) == &BoardStates::Empty {
          Some(*coords)
        } else {
          None
        }
      })
      .ok_or_else(|| "No open edge around 'most_recent_chosen_coords'".to_string())
  } else {
    bot.auto_play(gameboard)
  }
}

fn center_edge_check_placed_edge_near(
  bot: &Bot,
  gameboard: &BoardConfig,
) -> Result<Coordinates, String> {
  bot
    .most_recent_chosen_coords
    .as_ref()
    .unwrap()
    .get_edges_around_corner(gameboard)
    .iter()
    .find_map(|coords| {
      if gameboard.get_board_state(coords) == &BoardStates::Empty {
        Some(*coords)
      } else {
        None
      }
    })
    .ok_or_else(|| "No open edge around bot corner".to_string())
}

fn center_edge_check_placed_corner_then_edge(
  bot: &mut Bot,
  gameboard: &BoardConfig,
) -> Result<Coordinates, String> {
  bot.path = CurrentPath::DoubleWinCondition;

  let coords_around_player_edge = gameboard
    .last_modified_tile
    .unwrap()
    .get_corners_around_edge(gameboard);

  let possible_open_corner_near_player_edge = coords_around_player_edge.iter().find_map(|coords| {
    if gameboard.get_board_state(coords) != &BoardStates::Empty {
      Some(gameboard.get_board_state(coords))
    } else {
      None
    }
  });

  if possible_open_corner_near_player_edge != Some(&bot.bot_symbol) {
    // this is when the edge the player placed is far from the bot's corner
    bot.auto_play(gameboard)
  } else {
    center_edge_check_placed_edge_near(bot, gameboard)
  }
}

fn center_position_checks(bot: &mut Bot, gameboard: &BoardConfig) {
  match gameboard.get_board_position(&gameboard.last_modified_tile.unwrap()) {
    BoardPositions::Corner => {
      bot.most_recent_chosen_coords = bot.center_corner_checks(gameboard);
    }
    BoardPositions::Edge => {
      bot.most_recent_chosen_coords = bot.center_edge_checks(gameboard);
    }
    _ => bot.most_recent_chosen_coords = Err("Unknown board position".to_string()),
  }
}
