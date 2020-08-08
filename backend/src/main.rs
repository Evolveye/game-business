mod cactu_server;
mod game;
mod test;

use cactu_server::{ Server};
use serde::ser::{ Serialize, Serializer, SerializeSeq };
use game::socket_configurer;

#[tokio::main]
async fn main() {
  println!( "\n" );

  let server = Server::new();

  server.set_ws_configurer( socket_configurer() );
  // server.add_ws_room( events::GameRoom::new( sockets_data.game ) );
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
