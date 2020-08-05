use super::mime_types::get_mime_type;
use std::fs;
use futures::prelude::*;
use hyper::{
  header::{ self, AsHeaderName, HeaderMap, HeaderValue },
  // upgrade::OnUpgrade,
  Method, StatusCode, Body, Request, Response
};
use tokio_tungstenite::{
  tungstenite::protocol::{ Message, Role },
  WebSocketStream
};
use sha1::{ Digest, Sha1 };

pub const FRONTEND_BUILD_PATH: &str = "../frontend/build/";

pub async fn service( req:Request<Body> ) -> Result<Response<Body>, hyper::Error> {
  let mut response = Response::new( Body::empty() );
  let uri_path = req.uri().path();
  let mime_type = get_mime_type( uri_path );
  let headers = req.headers();

  if headers.contains_key( header::UPGRADE ) {
    match header_value( headers, header::UPGRADE ).as_str() {
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

        tokio::spawn( async {
          let upgraded = req.into_body()
            .on_upgrade().await
            .unwrap();
          let ws_stream = WebSocketStream::from_raw_socket( upgraded, Role::Server, None );
          let ws = ws_stream.await;
          let (mut _sink, mut stream) = ws.split();

          loop {
            let msg = stream.next().await.unwrap();

            println!( " > ws msg: {}", msg.unwrap() )
          }
        } );

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
fn header_value<T:AsHeaderName>( headers:&HeaderMap<HeaderValue>, name:T ) -> String {
  headers.get( name )
    .and_then( |v| v.to_str().ok() )
    .map( |v| v.to_lowercase() )
    .unwrap_or( String::new() )
}