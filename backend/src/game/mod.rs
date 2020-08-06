
use crate::http::{ Handler, Socket, Room, Value, json };
use crate::game::{ Game, GameEvent };

impl<T> Handler for Socket<T> {
  fn on_connection( &self ) {
    println!( " [i]  {} connected", self.get_id() );
  }
  fn on_disconnection( &self ) {
    println!( " [i]  {} disconnected", self.get_id() );
  }
  fn on_receive_data( &self, data:String ) {
    let value = json!( data );

    let game_event: GameEvent = serde_json::from_str( data.as_str() )
      .unwrap_or( GameEvent::Nothing );

    match game_event {
      GameEvent::SearchGame( board_type ) => {
        let board = if let Some( board ) = self.game.find_opened_board( &board_type ) {
          board
        } else {
          println!( " i Create new board" );
          self.game.create_board( board_type )
        };

        json!( { "event":"founded game", "data":board } )
      },

      GameEvent::Ping => json!( { "event":"pong" } ),
      GameEvent::Nothing => json!( Value::Null ),
    }
  }
}
pub struct GameRoom {
  game: Game,
}

impl GameRoom {
  pub fn new( game:Game ) -> GameRoom {
    GameRoom { game }
  }
}

impl Room for GameRoom {
  fn events_handler( &mut self, data:String ) -> Value {
    let game_event: GameEvent = serde_json::from_str( data.as_str() )
      .unwrap_or( GameEvent::Nothing );

    match game_event {
      GameEvent::SearchGame( board_type ) => {
        let board = if let Some( board ) = self.game.find_opened_board( &board_type ) {
          board
        } else {
          println!( " i Create new board" );
          self.game.create_board( board_type )
        };

        json!( { "event":"founded game", "data":board } )
      },

      GameEvent::Ping => json!( { "event":"pong" } ),
      GameEvent::Nothing => json!( Value::Null ),
    }
  }
}