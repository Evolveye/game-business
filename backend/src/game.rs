use crate::vec_serialize;
use serde::{ Serialize, Deserialize };

pub struct Game {
  boards: Vec<Board>,
}

impl Game {
  pub fn new() -> Game {
    Game { boards:vec![] }
  }

  pub fn create_board( &mut self, board_type:BoardType ) -> &Board {
    self.boards.push( Board::new( board_type ) );
    &self.boards.last().unwrap()
  }

  pub fn find_opened_board( &self, _board_type:&BoardType ) -> Option<&Board> {
    self.boards.first()
  }
}

#[derive( Deserialize )]
pub enum BoardType {
  Square( i8 ),
}

#[derive( Serialize )]
pub struct Board {
  #[serde( serialize_with="vec_serialize" )]
  tiles: Vec<Tile>,
}
impl Board {
  pub fn new( board_type:BoardType ) -> Board {
    match board_type {
      BoardType::Square( size) => Board {
        tiles: (1..size).map( |_| Tile::new() ).collect()
      }
    }
  }
}


#[derive( Serialize )]
struct Tile {}
impl Tile {
  fn new() -> Tile {
    Tile {}
  }
}

#[derive( Deserialize )]
#[serde( rename_all( deserialize="camelCase" ) )]
pub enum GameEvent {
  Nothing,
  Ping,
  SearchGame( BoardType ),
}