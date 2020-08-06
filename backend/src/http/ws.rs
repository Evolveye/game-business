use std::sync::Arc;
use futures::lock::Mutex;
use hyper::{ upgrade::Upgraded, Body, Request };
pub use serde_json::{ Value, json };
use futures::{
  stream::{ SplitSink, SplitStream },
  executor::block_on,
  prelude::*,
};
use tokio_tungstenite::{
  tungstenite::protocol::{ Message, Role },
  WebSocketStream
};

pub struct WebSocketController {
  storage: Arc<Mutex<Storage>>,

}
impl WebSocketController {
  pub fn new() -> WebSocketController {
    WebSocketController {
      // sockets: Vec::new(),
      storage: Arc::new( Mutex::new( Storage::new() ) ),
    }
  }

  pub fn handle_socket_from_request( &self, request:Request<Body> ) {
    let storage_mutex = Arc::clone( &self.storage );

    tokio::spawn( async move {
      let upgraded = request.into_body()
        .on_upgrade().await
        .unwrap();
      let ws_stream = WebSocketStream::from_raw_socket( upgraded, Role::Server, None );
      let ws_stream = ws_stream.await;
      let socket_mutex = Arc::new( Mutex::new( Socket::new( ws_stream ) ) );

      storage_mutex.lock().await.sockets.push( Arc::clone( &socket_mutex ) );

      let mut socket = socket_mutex.lock().await;

      loop {
        let msg = socket.wait_for_message().await;

        println!( " > ws msg: {}", msg )
      }
    } );
  }
  pub fn add_room<T:Room + Send + 'static>( &self, room:T ) {
    let mut storage = block_on( self.storage.lock() );

    storage.rooms.push( Arc::new( Mutex::new( room ) ) );
  }
}

struct Storage {
  sockets: Vec<Arc<Mutex<Socket>>>,
  rooms: Vec<Arc<Mutex<dyn Room + Send>>>,
}
impl Storage {
  pub fn new() -> Storage {
    Storage {
      sockets: Vec::new(),
      rooms: Vec::new(),
    }
  }
}

pub struct Socket {
  sink: SplitSink<WebSocketStream<Upgraded>, Message>,
  stream: SplitStream<WebSocketStream<Upgraded>>,
}
impl Socket {
  pub fn new( ws_stream:WebSocketStream<Upgraded> ) -> Socket {
    let (sink, stream) = ws_stream.split();

    Socket { sink, stream }
  }

  pub async fn wait_for_message( &mut self ) -> Message {
    self.stream.next().await.unwrap().unwrap()
  }
  pub fn send( &mut self, message:String ) {
    self.sink.send( message.into() );
  }
}

// #[derive(Send)]
pub trait Room {
  fn events_handler( &mut self, data:String ) -> Value;
}
// impl Server {
//   pub fn new() -> Server {
//     Server {
//       storage: Rc::new( RefCell::new( ServerStorage { rooms:Vec::new() } ) ),
//     }
//   }


//   pub async fn run( &self, addr:SocketAddr ) {
//     println!( "WS server ready on ws://{}", addr );

//     listen( addr, |out| {
//       println!( "Web socket spawning" );

//       // move|msg| out.send(msg)
//       Socket { server_storage:Rc::clone( &self.storage ), out }
//     } ).unwrap();
//   }
// }

// struct Socket {
//   out: Sender,
//   server_storage: Rc<RefCell<ServerStorage>>,
// }
// impl Handler for Socket {
//   fn on_message( &mut self, msg: Message) -> Result<(), Error> {
//     let storage = self.server_storage.borrow();
//     let json = msg.into_text().unwrap();

//     println!( " > {}", json );

//     for room in storage.rooms.iter() {
//       let response = room.borrow_mut().events_handler( json.clone() );

//       if response != Value::Null {
//         let response = response.to_string();

//         println!( " < {}", response );

//         let _ = self.out.send( Message::from( response.to_string() ) );
//       }
//     }

//     Ok(())
//   }
// }

// pub trait Room {
//   fn events_handler( &mut self, data:String ) -> Value;
// }