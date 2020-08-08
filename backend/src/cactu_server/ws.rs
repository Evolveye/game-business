use std::{
  // collections::HashMap,
  time::{ SystemTime, UNIX_EPOCH },
  sync::Arc,
};
use futures::lock::{ Mutex, MutexGuard };
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
  storage: Arc<Mutex<Storage<'static>>>,
}
impl WebSocketController {
  pub fn new() -> WebSocketController {
    WebSocketController {
      storage: Arc::new( Mutex::new( Storage::new() ) ),
    }
  }

  pub fn handle_socket_from_request( &self, request:Request<Body> ) {
    let storage_mutex = Arc::clone( &self.storage );

    tokio::spawn( async move {
      let mut storage_guard = storage_mutex.lock().await;
      let upgraded = request.into_body()
        .on_upgrade().await
        .unwrap();
      let ws_stream = WebSocketStream::from_raw_socket( upgraded, Role::Server, None );
      let ws_stream = ws_stream.await;
      let socket_mutex = Arc::new( Mutex::new( Socket::new( ws_stream ) ) );

      // let rooms = &storage_guard.rooms;
      let mut socket = socket_mutex.lock().await;

      storage_guard.sockets.push( Arc::clone( &socket_mutex ) );
      if let Some( mut configurer ) = storage_guard.socket_configurer.take() {
        configurer( &mut socket );
        storage_guard.socket_configurer = Some( configurer );
      }

      drop( storage_guard );

      loop {
        let msg = socket.wait_for_message().await;

        match msg {
          Message::Pong(_) |
          Message::Binary(_) |
          Message::Ping(_) => (),
          Message::Close(_) => {
            let id = socket.id;

            if let Some( mut on_disconnect ) = socket.on_disconnection_handler.take() {
              on_disconnect( &mut socket );
            }

            drop( socket );

            storage_mutex.lock().await.sockets.retain( |s| block_on( s.lock() ).id != id );

            break
          },
          Message::Text( _message ) => (),
        }

        // for room in rooms {
        //   room.lock().await.events_handler( msg.clone().into_text().unwrap() );
        // }

      }
    } );
  }
  pub fn set_ws_configurer( &self, configurer:Box<dyn FnMut( &mut MutexGuard<Socket> ) + Send + 'static> ) {
    let mut storage = block_on( self.storage.lock() );

    storage.socket_configurer = Some( Box::new( configurer ) );
  }
  pub fn add_room<U:Room + Send + 'static>( &self, room:U ) {
    let mut storage = block_on( self.storage.lock() );

    storage.rooms.push( Arc::new( Mutex::new( room ) ) );
  }
}

struct Storage<'a> {
  sockets: Vec<Arc<Mutex<Socket<'a>>>>,
  rooms: Vec<Arc<Mutex<dyn Room + Send>>>,
  socket_configurer: Option<Box<dyn FnMut( &mut MutexGuard<Socket> ) + Send>>,
}
impl<'a> Storage<'a> {
  pub fn new() -> Storage<'a> {
    Storage {
      sockets: Vec::new(),
      rooms: Vec::new(),
      socket_configurer: None,
    }
  }
}

pub struct Socket<'a> {
  id: u128,
  sink: SplitSink<WebSocketStream<Upgraded>, Message>,
  stream: SplitStream<WebSocketStream<Upgraded>>,
  on_message_handler: Option<Box<dyn FnMut() + Send + 'a>>,
  on_disconnection_handler: Option<Box<dyn FnMut( &mut Self ) + Send + 'a>>,
}
impl<'a> Socket<'a> {
  pub fn new( ws_stream:WebSocketStream<Upgraded> ) -> Socket<'a> {
    let (sink, stream) = ws_stream.split();
    let id = SystemTime::now().duration_since( UNIX_EPOCH ).unwrap().as_millis();

    Socket {
      id,
      sink,
      stream,
      on_message_handler: None,
      on_disconnection_handler: None,
    }
  }

  pub async fn wait_for_message( &mut self ) -> Message {
    self.stream.next().await.unwrap().unwrap()
  }

  pub fn get_id( &self ) -> u128 {
    self.id
  }
  pub fn send( & mut self, message:String ) {
    block_on( self.sink.send( message.into() ) );
  }
  pub fn broadcast( &mut self, message:String ) {
    todo!();
  }
  pub fn on_message( &mut self, handler:Box<impl FnMut( String )> ) {
    todo!();
  }
  pub fn on_disconnection( & mut self, handler:impl FnMut( &mut Self ) + Send + 'a ) {
    self.on_disconnection_handler = Some( Box::new( handler ) );
  }
}
impl PartialEq for Socket<'_> {
  fn eq( &self, other:&Socket ) -> bool {
    self.id == other.id
  }
}

pub trait Room {}