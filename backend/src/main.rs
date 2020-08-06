mod http;
mod ws_server;
mod events;
mod game;

// use game::{ Game };
// use ws_server::{ Server };
use serde::ser::{ Serialize, Serializer, SerializeSeq };

#[tokio::main]
async fn main() {
  println!( "\n" );

  http::run( ([127, 0, 0, 1], 80).into(), Some( ([91, 231, 24, 247], 80).into() ) ).await
}

fn vec_serialize<T,S>( vec:&Vec<T>, serializer:S ) -> Result<S::Ok, S::Error>
where S:Serializer, T:Serialize {
  let mut seq = serializer.serialize_seq( Some( vec.len() ) )?;

  for element in vec {
      seq.serialize_element( &element )?;
  }

  seq.end()
}