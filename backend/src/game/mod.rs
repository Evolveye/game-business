mod events;

use futures::lock::MutexGuard;
use crate::cactu_server::{ Socket };

pub fn socket_configurer<'a>() -> impl FnMut( &mut MutexGuard<Socket> ) + Send + Sync + 'a {
  |socket| {
    let id = socket.get_id();

    println!( " [i] {}::connected", id );

    socket.send( format!( "Connected succesfully with id {}", id ) );
    socket.on_disconnection( |s| println!( " [i] {}::disconnected", s.get_id() ) );
  }
}