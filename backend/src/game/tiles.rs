use serde::{ Serialize };

#[derive( Serialize )]
#[serde( rename_all( serialize="camelCase" ) )]
pub enum TileType {
  Start,
  Jail,
  Parking,
  GoToJail,
  City( i8, i32, i16, String ),
}

#[derive( Serialize )]
#[serde( rename_all( serialize="camelCase" ) )]
pub struct Tile {
  type_enum: TileType,
}
impl Tile {
  pub fn new( type_enum:TileType ) -> Tile {
    Tile { type_enum }
  }
}