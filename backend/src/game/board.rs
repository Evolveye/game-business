use rand::Rng;
use serde::{
  Serialize,
  Serializer,
  ser::SerializeSeq,
  Deserialize
};

use super::{ Snowflake, SnowFlakeTrait };
use super::tiles::*;

#[derive( Serialize )]
#[serde( rename_all( serialize="camelCase" ) )]
pub struct Board {
  #[serde( serialize_with="serialize_snowflake" )]
  id: Snowflake,
  board_type: BoardType,
  #[serde( serialize_with="serialize_vec" )]
  tiles: Vec<Tile>,
  players: Vec<Player>
}
impl Board {
  pub fn new( board_type:BoardType ) -> Board {
    let tiles = match board_type {
      BoardType::Square( 5 ) => {
        let tiles = vec![
          // // First edge
          // Tile::new( TileType::Start ),

          // Tile::new( TileType::City( 1, 0xd84e9c, "Patusy alkoholowe 1".to_owned() ) ),
          // Tile::new( TileType::City( 1, 0xd84e9c, "Patusy alkoholowe 2".to_owned() ) ),
          // Tile::new( TileType::City( 2, 0xfb9942, "Domki z zamurowanymi oknami 1".to_owned() ) ),

          // // Second edge
          // Tile::new( TileType::Jail ),

          // Tile::new( TileType::City( 2, 0xfb9942, "Domki z zamurowanymi oknami 2".to_owned() ) ),
          // Tile::new( TileType::City( 3, 0xfc3e3f, "Czarna dzielnia 1".to_owned() ) ),
          // Tile::new( TileType::City( 3, 0xfc3e3f, "Czarna dzielnia 2".to_owned() ) ),

          // // Third edge
          // Tile::new( TileType::Parking ),

          // Tile::new( TileType::City( 4, 0xf7f844, "Kuweta 1".to_owned() ) ),
          // Tile::new( TileType::City( 3, 0xfc3e3f, "Czarna dzielnia 3".to_owned() ) ),
          // Tile::new( TileType::City( 4, 0xf7f844, "Kuweta 2".to_owned() ) ),

          // // Fourth edge
          // Tile::new( TileType::GoToJail ),

          // Tile::new( TileType::City( 5, 0x0171ae, "Pola jagodowe 1".to_owned() ) ),
          // Tile::new( TileType::City( 5, 0x0171ae, "Pola jagodowe 2".to_owned() ) ),
          // Tile::new( TileType::City( 5, 0x0171ae, "Pola jagodowe 3".to_owned() ) ),
        ];

        if tiles.len() != 16 {
          panic!( "BoardType with size 5 should had 16 tiles!" );
        }

        tiles
      },
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

        tiles
      },
      BoardType::Square( _ ) => Vec::new()
    };

    Board {
      id: Snowflake::new(),
      board_type,
      tiles,
      players: Vec::new(),
    }
  }

  pub fn get_type( &self ) -> BoardType {
    self.board_type
  }
  pub fn get_id( &self ) -> Snowflake {
    self.id
  }

  pub fn add_player( &mut self, player_id:Snowflake ) {
    self.players.push( Player::new( player_id, self.id ) )
  }
  pub fn remove_player( &mut self, player_id:Snowflake ) {
    self.players.retain( |p| p.id == player_id )
  }
  pub fn move_player_by( &mut self, player_id:Snowflake, move_value:u8 ) -> Result<u8,String> {
    if let Some( player ) = self.players.iter_mut().find( |p| p.id == player_id ) {
      let new_tile_index = (move_value + player.tile_index) % self.tiles.len() as u8;

      player.move_to( new_tile_index );

      Ok( new_tile_index )
    } else {
      Err( format!( "Player with id {} not found", player_id.to_string() ) )
    }
  }
}

#[derive( Deserialize, Serialize, PartialEq, Copy, Clone )]
#[serde( rename_all( deserialize="camelCase", serialize="camelCase" ) )]
pub enum BoardType {
  Square( i8 ),
}

#[derive( Serialize )]
#[serde( rename_all( deserialize="camelCase", serialize="camelCase" ) )]
pub struct Player {
  #[serde( serialize_with="serialize_snowflake" )]
  id: Snowflake,
  #[serde( serialize_with="serialize_snowflake" )]
  board_id: Snowflake,
  tile_index: u8,
  color: u32,
}
impl Player {
  fn new( id:Snowflake, board_id:Snowflake ) -> Player {
    Player {
      id,
      board_id,
      tile_index: 0,
      color: rand::thread_rng().gen_range( 1, 1 << 24 ),
    }
  }
  fn move_to( &mut self, tile_index:u8 ) {
    self.tile_index = tile_index;
  }
}

fn serialize_snowflake<S>( snowflake:&Snowflake, serializer:S ) -> Result<S::Ok, S::Error>
where S: Serializer {
  serializer.serialize_str( snowflake.to_string().as_str() )
}
fn serialize_vec<T,S>( vec:&Vec<T>, serializer:S ) -> Result<S::Ok, S::Error>
where S:Serializer, T:Serialize {
  let mut seq = serializer.serialize_seq( Some( vec.len() ) )?;

  for element in vec {
    seq.serialize_element( &element )?;
  }

  seq.end()
}