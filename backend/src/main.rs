mod http;
mod ws_server;
mod events;
mod game;

use http::Server;
use game::Game;
use std::sync::Arc;
use serde::ser::{ Serialize, Serializer, SerializeSeq };

struct SocketData {
  game: Game,
}

#[tokio::main]
async fn main() {
  println!( "\n" );

  let ws_handler =

  let sockets_data = SocketData {
    game: Game::new(),
  };
  let server = Server::new( sockets_data );

  server.add_ws_room( events::GameRoom::new( sockets_data.game ) );
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