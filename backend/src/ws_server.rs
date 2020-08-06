// use ws::{ listen, Sender, Handler, Message, Error };
// use std::net::SocketAddr;
// use std::rc::Rc;
// use std::cell::RefCell;

// pub use serde_json::{ Value, json };

// struct ServerStorage {
//   rooms: Vec<Rc<RefCell<dyn Room>>>,
// }
// pub struct Server {
//   storage: Rc<RefCell<ServerStorage>>,
// }

// impl Server {
//   pub fn new() -> Server {
//     Server {
//       storage: Rc::new( RefCell::new( ServerStorage { rooms:Vec::new() } ) ),
//     }
//   }

//   pub fn add_room<T:Room + 'static>( &self, room:T ) {
//     self.storage.borrow_mut().rooms.push( Rc::new( RefCell::new( room ) ) )
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
