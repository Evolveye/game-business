use crate::vec_serialize;
use serde::{ Serialize, Deserialize };

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
      // BoardType::Square( 5 ) => {
      //   let tiles = vec![
      //     // First edge
      //     Tile::new( TileType::Start ),

      //     Tile::new( TileType::City( 1, 0xd84e9c, "Patusy alkoholowe 1".to_owned() ) ),
      //     Tile::new( TileType::City( 1, 0xd84e9c, "Patusy alkoholowe 2".to_owned() ) ),
      //     Tile::new( TileType::City( 2, 0xfb9942, "Domki z zamurowanymi oknami 1".to_owned() ) ),

      //     // Second edge
      //     Tile::new( TileType::Jail ),

      //     Tile::new( TileType::City( 2, 0xfb9942, "Domki z zamurowanymi oknami 2".to_owned() ) ),
      //     Tile::new( TileType::City( 3, 0xfc3e3f, "Czarna dzielnia 1".to_owned() ) ),
      //     Tile::new( TileType::City( 3, 0xfc3e3f, "Czarna dzielnia 2".to_owned() ) ),

      //     // Third edge
      //     Tile::new( TileType::Parking ),

      //     Tile::new( TileType::City( 4, 0xf7f844, "Kuweta 1".to_owned() ) ),
      //     Tile::new( TileType::City( 3, 0xfc3e3f, "Czarna dzielnia 3".to_owned() ) ),
      //     Tile::new( TileType::City( 4, 0xf7f844, "Kuweta 2".to_owned() ) ),

      //     // Fourth edge
      //     Tile::new( TileType::GoToJail ),

      //     Tile::new( TileType::City( 5, 0x0171ae, "Pola jagodowe 1".to_owned() ) ),
      //     Tile::new( TileType::City( 5, 0x0171ae, "Pola jagodowe 2".to_owned() ) ),
      //     Tile::new( TileType::City( 5, 0x0171ae, "Pola jagodowe 3".to_owned() ) ),
      //   ];

      //   if tiles.len() != 16 {
      //     panic!( "BoardType with size 5 should had 16 tiles!" );
      //   }

      //   Board { board_type, tiles }
      // },
      BoardType::Square( 9 ) => {
        let tiles = vec![
          // First edge
          Tile::new( TileType::Start ),

          Tile::new( TileType::City( 1, 0xd84e9c, 75, "Granada".to_owned() ) ),
          Tile::new( TileType::City( 1, 0xd84e9c, 125, "Seville".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 145, "Madrid".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 100, "Haldives".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 130, "Kongo".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 150, "Beijlino".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 165, "Shanghai".to_owned() ) ),

          // Second edge
          Tile::new( TileType::Jail ),

          Tile::new( TileType::City( 2, 0xfb9942, 190, "Venice".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 210, "Milan".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 240, "Rome".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 345, "Gniewino".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 235, "Hamburg".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 260, "Cyprus".to_owned() ) ),
          Tile::new( TileType::City( 2, 0xfb9942, 275, "Berlin".to_owned() ) ),

          // Third edge
          Tile::new( TileType::Parking ),

          Tile::new( TileType::City( 4, 0xf7f844, 390, "London".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 310, "Sochi".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 340, "Sydney".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 345, "Poznań".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 335, "Chickago".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 360, "Vegas".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 375, "New York".to_owned() ) ),

          // Fourth edge
          Tile::new( TileType::GoToJail ),

          Tile::new( TileType::City( 4, 0xf7f844, 490, "Adobe".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 410, "Lyon".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 440, "Paris".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 445, "Śląsk".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 435, "Kazan".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 460, "Cegielnia".to_owned() ) ),
          Tile::new( TileType::City( 4, 0xf7f844, 475, "Mobyn".to_owned() ) ),
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
  City( i8, i32, i16, String ),
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