use serde_json::{ json };
use super::types::{ GameEvent, Board, BoardType };
use crate::cactu_server::Socket;

pub fn event_handler( socket:&mut Socket, event:GameEvent ) {
  match  event {
    GameEvent::Nothing => (),
    GameEvent::Ping => {
      let id = format!( "{}", socket.get_id() );

      socket.send( json!( { "event":"pong", "data":id } ).to_string().as_str() );
    },
    GameEvent::SearchGame( BoardType::Square( size) ) => {
      let board = Board::new( BoardType::Square( size ) );

      socket.send( json!( { "event":"founded game", "data":board } ).to_string().as_str() );
    }
  }
}