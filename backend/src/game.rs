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

  pub fn find_opened_board( &self, board_type:&BoardType ) -> Option<&Board> {
    self.boards.iter().find( |board| board.board_type == *board_type )
  }
}

#[derive( Deserialize, Serialize, PartialEq )]
#[serde( rename_all( deserialize="camelCase", serialize="camelCase" ) )]
pub enum BoardType {
  Square( i8 ),
}

#[derive( Serialize )]
#[serde( rename_all( serialize="camelCase" ) )]
pub struct Board {
  board_type: BoardType,
  #[serde( serialize_with="vec_serialize" )]
  tiles: Vec<Tile>,
}
impl Board {
  pub fn new( board_type:BoardType ) -> Board {
    match board_type {
      BoardType::Square( 5 ) => {
        let tiles = vec![
          // First edge
          Tile::new( TileType::Start ),

          Tile::new( TileType::City( 1, "#d84e9c".to_owned(), "Patusy alkoholowe 1".to_owned() ) ),
          Tile::new( TileType::City( 1, "#d84e9c".to_owned(), "Patusy alkoholowe 2".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Domki z zamurowanymi oknami 1".to_owned() ) ),

          // Second edge
          Tile::new( TileType::Jail ),

          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Domki z zamurowanymi oknami 2".to_owned() ) ),
          Tile::new( TileType::City( 3, "#fc3e3f".to_owned(), "Czarna dzielnia 1".to_owned() ) ),
          Tile::new( TileType::City( 3, "#fc3e3f".to_owned(), "Czarna dzielnia 2".to_owned() ) ),

          // Third edge
          Tile::new( TileType::Parking ),

          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Kuweta 1".to_owned() ) ),
          Tile::new( TileType::City( 3, "#fc3e3f".to_owned(), "Czarna dzielnia 3".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Kuweta 2".to_owned() ) ),

          // Fourth edge
          Tile::new( TileType::GoToJail ),

          Tile::new( TileType::City( 5, "#0171ae".to_owned(), "Pola jagodowe 1".to_owned() ) ),
          Tile::new( TileType::City( 5, "#0171ae".to_owned(), "Pola jagodowe 2".to_owned() ) ),
          Tile::new( TileType::City( 5, "#0171ae".to_owned(), "Pola jagodowe 3".to_owned() ) ),
        ];

        if tiles.len() != 16 {
          panic!( "BoardType with size 5 should had 16 tiles!" );
        }

        Board { board_type, tiles }
      },
      BoardType::Square( 9 ) => {
        let tiles = vec![
          // First edge
          Tile::new( TileType::Start ),

          Tile::new( TileType::City( 1, "#d84e9c".to_owned(), "Granada".to_owned() ) ),
          Tile::new( TileType::City( 1, "#d84e9c".to_owned(), "Seville".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Madrid".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Haldives".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Kongo".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Beijlino".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Shanghai".to_owned() ) ),

          // Second edge
          Tile::new( TileType::Jail ),

          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Venice".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Milan".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Rome".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Gniewino".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Hamburg".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Cyprus".to_owned() ) ),
          Tile::new( TileType::City( 2, "#fb9942".to_owned(), "Berlin".to_owned() ) ),

          // Third edge
          Tile::new( TileType::Parking ),

          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "London".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Sochi".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Sydney".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Poznań".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Chickago".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Vegas".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "New York".to_owned() ) ),

          // Fourth edge
          Tile::new( TileType::GoToJail ),

          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Adobe".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Lyon".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Paris".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Śląsk".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Kazan".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Cegielnia".to_owned() ) ),
          Tile::new( TileType::City( 4, "#f7f844".to_owned(), "Mobyn".to_owned() ) ),
        ];

        if tiles.len() != 32 {
          panic!( "BoardType with size 9 should had 32 tiles!" );
        }

        Board { board_type, tiles }
      },
      BoardType::Square( size ) => {
        Board {
          board_type: BoardType::Square( size ),
          tiles: Vec::new(),
        }
      }
    }
  }
}

#[derive( Serialize )]
#[serde( rename_all( serialize="camelCase" ) )]
enum TileType {
  Start,
  Jail,
  Parking,
  GoToJail,
  City( i8, String, String ),
}

#[derive( Serialize )]
#[serde( rename_all( serialize="camelCase" ) )]
struct Tile {
  type_enum: TileType,
}
impl Tile {
  fn new( type_enum:TileType ) -> Tile {
    Tile { type_enum }
  }
}

#[derive( Deserialize )]
#[serde( rename_all( deserialize="camelCase" ) )]
pub enum GameEvent {
  Nothing,
  Ping,
  SearchGame( BoardType ),
}