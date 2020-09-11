mod board;
mod types;

use futures::lock::MutexGuard;
use std::sync::{ Arc, Mutex };
use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };

use board::*;
use types::*;
use crate::cactu_server::{ Snowflake, Socket };

pub struct Game {
  players_in_lobby: Vec<Player>,
  boards: Vec<Board>,
}

impl Game {
  pub fn new() -> Game {
    Game {
      players_in_lobby: Vec::new(),
      boards: Vec::new(),
    }
  }
  pub fn new_mutex() -> Arc<Mutex<Game>> {
    Arc::new( Mutex::new( Game::new() ) )
  }

  fn create_player_with_id( &mut self, id:Snowflake ) {
    self.players_in_lobby.push( Player( id ) )
  }
  fn remove_player( &mut self, id:Snowflake ) {
    self.players_in_lobby.retain( |p| p.0 == id )
  }

  fn player_want_to_join_to( &mut self, board_type:BoardType ) -> Result<Value,()> {
    if let Some( board ) = self.boards.iter().find( |b| b.get_type() == board_type ) {
      Ok(json!( board ))
    } else {
      let board = Board::new( board_type );
      let json = json!( board );

      self.boards.push( board );
      Ok( json )
    }
  }
  // pub fn create_board( &mut self, board_type:BoardType ) -> &Board {
  //   self.boards.push( Board::new( board_type ) );
  //   &self.boards.last().unwrap()
  // }

  // pub fn find_opened_board( &self, board_type:&BoardType ) -> Option<&Board> {
  //   self.boards.iter().find( |board| board.board_type == *board_type )
  // }

  pub fn socket_configurer<'a>( game_mutex:Arc<Mutex<Game>> ) -> impl FnMut( &mut MutexGuard<Socket> ) + Send + Sync + 'a {
    let game_mutex = Arc::clone( &game_mutex );

    move |socket:&mut MutexGuard<Socket>| {
      let game_mutex_disc = Arc::clone( &game_mutex );
      let game_mutex_conn = Arc::clone( &game_mutex );
      let id = socket.get_id();

      println!( " [i] {}::connected", id.to_string() );
      game_mutex.lock().unwrap().create_player_with_id( id );

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
            if let Ok( board_data ) = game.player_want_to_join_to( board_type ) {
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