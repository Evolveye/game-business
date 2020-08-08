mod mime_types;
mod ws;

use std::{
  sync::Arc,
  net::SocketAddr,
  fs,
};
use futures::lock::MutexGuard;
use ws::WebSocketController;
use mime_types::get_mime_type;
use sha1::{ Digest, Sha1 };
use hyper::{
  Server as HyperServer,
  header::{ self, AsHeaderName, HeaderMap, HeaderValue },
  service::{ make_service_fn, service_fn },
  Method, StatusCode, Body, Request, Response,
};

pub use ws::{ Room, Socket, Value, json };

const FRONTEND_BUILD_PATH: &str = "../frontend/build/";

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
          Server::service( req, Arc::clone( &clone ) )
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
  async fn service<'a>( req:Request<Body>, websocket_controller:Arc<WebSocketController> ) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new( Body::empty() );
    let uri_path = req.uri().path();
    let mime_type = get_mime_type( uri_path );
    let headers = req.headers();

    if headers.contains_key( header::UPGRADE ) {
      match Server::header_value( headers, header::UPGRADE ).as_str() {
        "websocket" => {
          let accept = {
            let mut hasher = Sha1::new();
            let key = req.headers().get( header::SEC_WEBSOCKET_KEY ).unwrap();

            hasher.update( String::from( key.to_str().unwrap() ) );
            hasher.update( "258EAFA5-E914-47DA-95CA-C5AB0DC85B11" );

            base64::encode( hasher.finalize() )
          };

          *response.status_mut() = StatusCode::SWITCHING_PROTOCOLS;
          response.headers_mut().insert( header::UPGRADE, "websocket".parse().unwrap() );
          response.headers_mut().insert( header::CONNECTION, "upgrade".parse().unwrap() );
          response.headers_mut().insert( header::SEC_WEBSOCKET_ACCEPT, accept.parse().unwrap() );

          websocket_controller.handle_socket_from_request( req );

          Ok( response )
        }
        _ => {
          println!( "NOT_IMPLEMENTED" );

          *response.status_mut() = StatusCode::NOT_IMPLEMENTED;
          *response.body_mut() = Body::from( "Server doesn not support that upgrade type" );

          Ok( response )
        }
      }
    } else {
      match (req.method(), uri_path) {
        (&Method::GET, _) => {
          let path = if mime_type == "" {
            response.headers_mut().insert( header::CONTENT_TYPE, "text/html".parse().unwrap() );
            format!( "{}/index.html", FRONTEND_BUILD_PATH )
          } else {
            response.headers_mut().insert( header::CONTENT_TYPE, "text/html".parse().unwrap() );
            format!( "{}{}", FRONTEND_BUILD_PATH, mime_type )
          };

          println!( "uri: {: <50} mime: {: <20} path: {:?}", uri_path, mime_type, path );

          let file = fs::read( path ).unwrap();

          *response.body_mut() = Body::from( file );

          Ok( response )
        },

        _ => {
          *response.status_mut() = StatusCode::NOT_FOUND;

          Ok( response )
        }
      }
    }
  }

  pub fn add_ws_room<T:Room + Send + 'static>( &self, room:T ) {
     self.websocket_controller_arc.add_room( room );
  }
  pub fn set_ws_configurer<'a>( &self, configurer:impl FnMut( &mut MutexGuard<Socket> ) + Send + Sync + 'a + 'static ) {
    self.websocket_controller_arc.set_ws_configurer( Box::new( configurer ) )
  }
  fn header_value<T:AsHeaderName>( headers:&HeaderMap<HeaderValue>, name:T ) -> String {
    headers.get( name )
      .and_then( |v| v.to_str().ok() )
      .map( |v| v.to_lowercase() )
      .unwrap_or( String::new() )
  }

}

