mod http;
mod ws_server;
mod events;
mod game;

use http::Server;
use game::Game;
use std::sync::Arc;
use serde::ser::{ Serialize, Serializer, SerializeSeq };

#[tokio::main]
async fn main() {
  println!( "\n" );

  let server = Server::new();
  let game = Game::new();

  server.add_ws_room( events::GameRoom::new( game ) );
  server.run( ([127, 0, 0, 1], 80).into(), Some( ([91, 231, 24, 247], 80).into() ) ).await
}

fn vec_serialize<T,S>( vec:&Vec<T>, serializer:S ) -> Result<S::Ok, S::Error>
where S:Serializer, T:Serialize {
  let mut seq = serializer.serialize_seq( Some( vec.len() ) )?;

  for element in vec {
      seq.serialize_element( &element )?;
  }

  seq.end()
}