use serde_json::{ Value, from_str };
use ws::{ listen, Sender as WsServer, Handler, Message, Error };
use std::net::SocketAddr;

struct Server {
  out: WsServer
}

impl Handler for Server {
  fn on_message( &mut self, msg: Message) -> Result<(),Error> {
    let string = msg.into_text().unwrap();
    let test = from_str( string.as_ref() )
      .unwrap_or( Value::Null );

    println!( "Server got message -> event: {}; data: {}", test[ "event" ], test[ "data" ] );

    self.out.send( string )
  }
}

pub async fn run( addr:SocketAddr ) {
  println!( "WebSocket ready on {}", addr );

  listen( addr, |out| {
    println!( "Web socket spawning" );

    Server { out }
  } ).unwrap();
}