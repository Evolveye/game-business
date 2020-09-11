mod events;
mod types;

use types::GameEvent;
use futures::lock::MutexGuard;
use crate::cactu_server::{ Socket };

pub fn socket_configurer<'a>() -> impl FnMut( &mut MutexGuard<Socket> ) + Send + Sync + 'a {
  |socket:&mut MutexGuard<Socket>| {
    let id = socket.get_id();

    println!( " [i] {}::connected", id );

    socket.send( format!( "Connected succesfully with id {}", id ).as_str() );
    socket.on_disconnection( |s| println!( " [i] {}::disconnected", s.get_id() ) );
    socket.on_message( |s, message| {
      let game_event: GameEvent = serde_json::from_str( message.as_str() )
        .unwrap_or_else( |_| {
          s.send( format!( "Wrong game event -- {}", message ).as_str() );

          GameEvent::Nothing
        } );

      events::event_handler( s, game_event );
    } )
  }
}