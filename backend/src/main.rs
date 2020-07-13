mod http;
mod ws_server;
mod events;
mod game;

use game::{ Game };
use ws_server::{ Server };
use serde::ser::{ Serialize, Serializer, SerializeSeq };

#[tokio::main]
async fn main() {
  let ws_server = Server::new();

  let game = Game::new();
  ws_server.add_room( events::GameRoom::new( game ) );

  println!( "\n" );

  futures::join!(
    http::run( ([127, 0, 0, 1], 80).into() ),
    ws_server.run( ([127, 0, 0, 1], 8080).into() ),
  );
}

fn vec_serialize<T,S>( vec:&Vec<T>, serializer:S ) -> Result<S::Ok, S::Error>
where S:Serializer, T:Serialize {
  let mut seq = serializer.serialize_seq( Some( vec.len() ) )?;

  for element in vec {
      seq.serialize_element( &element )?;
  }

  seq.end()
}