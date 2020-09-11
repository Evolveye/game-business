mod board;
mod tiles;

use futures::lock::MutexGuard;
use std::{
  collections::HashMap,
  sync::{ Arc, Mutex },
};
use serde::{ Serialize, Deserialize };
use serde_json::{ Value, json };

use board::*;
use crate::cactu_server::{ Snowflake, Socket };

pub struct Game {
  boards: HashMap<u64, Board>,
}

impl Game {
  pub fn new() -> Game {
    Game {
      boards: HashMap::new(),
    }
  }
  pub fn new_mutex() -> Arc<Mutex<Game>> {
    Arc::new( Mutex::new( Game::new() ) )
  }

  fn remove_player( &mut self, player_id:Snowflake ) {
    for board in self.boards.values_mut() {
      board.remove_player( player_id );
    }
  }
  fn player_want_to_join_to( &mut self, player_id:Snowflake, board_type:BoardType ) -> Result<Value,()> {
    if let Some( board ) = self.boards.values_mut().find( |b| b.get_type() == board_type ) {
      board.add_player( player_id );

      Ok(json!( board ))
    } else {
      let mut board = Board::new( board_type );

      board.add_player( player_id );

      let json = json!( board );

      self.boards.insert( board.get_id().get_value(), board );

      Ok( json )
    }
  }

  pub fn socket_configurer<'a>( game_mutex:Arc<Mutex<Game>> ) -> impl FnMut( &mut MutexGuard<Socket> ) + Send + Sync + 'a {
    let game_mutex = Arc::clone( &game_mutex );

    move |socket:&mut MutexGuard<Socket>| {
      let game_mutex_disc = Arc::clone( &game_mutex );
      let game_mutex_conn = Arc::clone( &game_mutex );
      let id = socket.get_id();

      println!( " [i] {}::connected", id.to_string() );

      socket.emit( format!( "Connected succesfully with id {}", id.to_string() ).as_str() );
      socket.on_disconnection( move |s| {
        game_mutex_disc.lock().unwrap().remove_player( s.get_id() );
        println!( " [i] {}::disconnected", s.get_id().to_string() );
      } );
      socket.on_message( move |s, message| {
        let mut game = game_mutex_conn.lock().unwrap();
        let game_event: GameEvent = serde_json::from_str( message.as_str() )
          .unwrap_or_else( |_| {
            s.emit( format!( "Wrong game event -- {}", message ).as_str() );

            GameEvent::Nothing
          } );

        match game_event {
          GameEvent::Nothing => (),
          GameEvent::Ping => Game::emit( s, "pong", s.get_id().to_string() ),
          GameEvent::SearchGame( board_type ) => {
            if let Ok( board_data ) = game.player_want_to_join_to( s.get_id(), board_type ) {
              Game::emit( s, "founded game", board_data )
            } else {
              Game::emit( s, "not founded game", board_type )
            }
          }
        }
      } )
    }
  }

  fn emit<'a>( socket:&mut Socket, event_name:&str, data:impl Serialize ) {
    socket.emit( json!( { "event":event_name, "data":data } ).to_string().as_str() )
  }
}

#[derive( Deserialize )]
#[serde( rename_all( deserialize="camelCase" ) )]
pub enum GameEvent {
  Nothing,
  Ping,
  SearchGame( BoardType ),
}