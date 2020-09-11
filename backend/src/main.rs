mod cactu_server;
mod game;
mod test;

use cactu_server::Server;
use serde::ser::{ Serialize, Serializer, SerializeSeq };
use game::Game;

#[tokio::main]
async fn main() {
  println!( "\n" );

  let server = Server::new();
  let game = Game::new_mutex();

  server.set_ws_configurer( Game::socket_configurer( game ) );
  // server.add_ws_room( events::GameRoom::new( sockets_data.game ) );
  server.run( ([0, 0, 0, 0], 3000).into(), Some( ([91, 231, 24, 247], 3000).into() ) ).await
}

fn vec_serialize<T,S>( vec:&Vec<T>, serializer:S ) -> Result<S::Ok, S::Error>
where S:Serializer, T:Serialize {
  let mut seq = serializer.serialize_seq( Some( vec.len() ) )?;

  for element in vec {
    seq.serialize_element( &element )?;
  }

  seq.end()
}
