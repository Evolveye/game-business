use std::sync::Arc;
use std::time::{ SystemTime, UNIX_EPOCH };
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

pub struct WebSocketController<T> {
  storage: Arc<Mutex<Storage<T>>>,
}
impl<T> WebSocketController<T> {
  pub fn new( socket_external_data:T ) -> WebSocketController<T> {
    WebSocketController {
      storage: Arc::new( Mutex::new( Storage::new( socket_external_data ) ) ),
    }
  }

  pub fn handle_socket_from_request( &self, request:Request<Body> ) {
    let storage_mutex = Arc::clone( &self.storage );

    tokio::spawn( async move {
      let storage_guard = storage_mutex.lock().await;
      let upgraded = request.into_body()
        .on_upgrade().await
        .unwrap();
      let ws_stream = WebSocketStream::from_raw_socket( upgraded, Role::Server, None );
      let ws_stream = ws_stream.await;
      let socket_mutex = Arc::new( Mutex::new( Socket::new( storage_guard.sockets_external_data, ws_stream ) ) );

      // let rooms = &storage_guard.rooms;
      let mut socket = socket_mutex.lock().await;

      storage_guard.sockets.push( Arc::clone( &socket_mutex ) );
      socket.on_connection();

      loop {
        let msg = socket.wait_for_message().await;

        match msg {
          Message::Pong(_) |
          Message::Binary(_) |
          Message::Ping(_) => (),
          Message::Close(_) => {
            let id = socket.id;

            socket.on_disconnection();

            drop( socket );

            storage_mutex.lock().await.sockets.retain( |s| block_on( s.lock() ).id != id );

            break
          },
          Message::Text( message ) => socket.on_receive_data( message ),
        }

        // for room in rooms {
        //   room.lock().await.events_handler( msg.clone().into_text().unwrap() );
        // }

      }
    } );
  }
  pub fn add_room<U:Room + Send + 'static>( &self, room:U ) {
    let mut storage = block_on( self.storage.lock() );

    storage.rooms.push( Arc::new( Mutex::new( room ) ) );
  }
}

struct Storage<T> {
  sockets: Vec<Arc<Mutex<Socket<T>>>>,
  rooms: Vec<Arc<Mutex<dyn Room + Send>>>,
  sockets_external_data: T,
}
impl<T> Storage<T> {
  pub fn new( sockets_external_data:T ) -> Storage<T> {
    Storage {
      sockets: Vec::new(),
      rooms: Vec::new(),
      sockets_external_data,
    }
  }
}
impl<T> PartialEq for Socket<T> {
  fn eq( &self, other:&Socket<T> ) -> bool {
    self.id == other.id
  }
}

pub struct Socket<T> {
  id: u128,
  external_data: T,
  sink: SplitSink<WebSocketStream<Upgraded>, Message>,
  stream: SplitStream<WebSocketStream<Upgraded>>,
  // server: SplitStream<WebSocketStream<Upgraded>>,
}
impl<T> Socket<T> {
  pub fn new( external_data:T, ws_stream:WebSocketStream<Upgraded> ) -> Socket<T> {
    let (sink, stream) = ws_stream.split();
    let id = SystemTime::now().duration_since( UNIX_EPOCH ).unwrap().as_millis();

    Socket { id, external_data, sink, stream }
  }

  pub async fn wait_for_message( &mut self ) -> Message {
    self.stream.next().await.unwrap().unwrap()
  }

  pub fn get_id( &self ) -> u128 {
    self.id
  }
  pub fn send( &mut self, message:String ) {
    self.sink.send( message.into() );
  }
  pub fn broadcast( &mut self, message:String ) {
    todo!();
  }
}
pub trait Handler {
  fn on_connection( &self ) {}
  fn on_disconnection( &self ) {}
  fn on_receive_data( &self, data:String );
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