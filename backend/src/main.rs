mod cactu_server;
mod game;
mod test;

use cactu_server::Server;
use game::Game;

#[tokio::main]
async fn main() {
  println!( "\n" );

  let server = Server::new();
  let game = Game::new_mutex();

  server.set_ws_configurer( Game::socket_configurer( game ) );
  // server.add_ws_room( events::GameRoom::new( sockets_data.game ) );
  server.run( ([0, 0, 0, 0], 3000).into(), Some( ([91, 231, 24, 247], 3000).into() ) ).await
}