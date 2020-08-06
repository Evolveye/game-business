mod mime_types;
mod service;
mod ws;

use std::sync::Arc;
use std::net::SocketAddr;
use hyper::Server as HyperServer;
use hyper::service::{ make_service_fn, service_fn };
use ws::WebSocketController;

pub use ws::{ Socket, Handler, Room, Value, json };

pub struct Server {
  websocket_controller_arc: Arc<WebSocketController>,
}
impl Server {
  pub fn new() -> Server {
    Server {
      websocket_controller_arc: Arc::new( WebSocketController::new() ),
    }
  }

  pub async fn run( &self, addr:SocketAddr, public_addr:Option<SocketAddr> ) {
    let websocket_controller = Arc::clone( &self.websocket_controller_arc );
    let service = make_service_fn( move |_| {
      let clone = Arc::clone( &websocket_controller );

      async move {
        Ok::<_, hyper::Error>( service_fn( move |req| {
          service::service( req, Arc::clone( &clone ) )
        } ) )
      }
    } );
    let shutdown_signal = || async {
      tokio::signal::ctrl_c().await
        .expect( "failed to install CTRL+C signal handler" );
    };

    let server = HyperServer::bind( &addr )
      .serve( service )
      .with_graceful_shutdown( shutdown_signal() );

    println!( "HTTP server ready on http://{}", addr );

    if let Some( public_addr ) = public_addr {
      println!( " - public address is http://{}", public_addr );
    }

    if let Err( e ) = server.await {
      eprintln!( "server error: {}", e );
    }
  }

  pub fn add_ws_room<T:Room + Send + 'static>( &self, room:T ) {
     self.websocket_controller_arc.add_room( room );
  }
}

