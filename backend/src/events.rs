
use crate::ws_server::{ Room, Value, json };
use crate::game::{ Game, GameEvent };

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
          self.game.create_board( board_type )
        };

        json!( { "event":"founded game", "data":board } )
      },

      GameEvent::Ping => json!( { "event":"pong" } ),
      GameEvent::Nothing => json!( Value::Null ),
    }
  }
}