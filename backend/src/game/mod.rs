mod types;

use futures::lock::MutexGuard;
use std::sync::{ Arc, Mutex };
use serde_json::{ json };
use types::*;
use crate::cactu_server::{ Socket };

pub struct Game {
  // boards: Vec<Board>,
}

impl Game {
  pub fn new() -> Game {
    Game {
      // boards: vec![]
    }
  }
  pub fn new_mutex() -> Arc<Mutex<Game>> {
    Arc::new( Mutex::new( Game::new() ) )
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
      let game_mutex = Arc::clone( &game_mutex );
      let id = socket.get_id().to_string();

      println!( " [i] {}::connected", id );

      socket.send( format!( "Connected succesfully with id {}", id ).as_str() );
      socket.on_disconnection( move |s| {
        // game_mutex.lock().unwrap().remove_player( s.get_id() );
        println!( " [i] {}::disconnected", s.get_id().to_string() );
      } );
      socket.on_message( move |s, message| {
        let _game_mutex = game_mutex.lock().unwrap();
        let game_event: GameEvent = serde_json::from_str( message.as_str() )
          .unwrap_or_else( |_| {
            s.send( format!( "Wrong game event -- {}", message ).as_str() );

            GameEvent::Nothing
          } );

          match game_event {
            GameEvent::Nothing => (),
            GameEvent::Ping => {
              let id = format!( "{}", s.get_id().to_string() );

              s.send( json!( { "event":"pong", "data":id } ).to_string().as_str() );
            },
            GameEvent::SearchGame( BoardType::Square( size) ) => {
              let board = Board::new( BoardType::Square( size ) );

              s.send( json!( { "event":"founded game", "data":board } ).to_string().as_str() );
            }
          }
        // events::event_handler( s, game_event );
      } )
    }
  }
}