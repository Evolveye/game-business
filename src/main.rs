use std::fs;
use hyper::{ Method, StatusCode, Body, Request, Response, Server };
use hyper::service::{ make_service_fn, service_fn };
use futures::TryStreamExt as _;

async fn service( req:Request<Body> ) -> Result<Response<Body>, hyper::Error> {
  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => {
      let file = fs::read( "./index.html" )
        .unwrap();

      Ok( Response::new( Body::from( file ) ) )
    },

    (&Method::POST, "/echo") => Ok( Response::new( req.into_body() ) ),

    (&Method::POST, "/echo/uppercase") => {
      let mapping = req.into_body().map_ok( |chunk| {
        chunk.iter()
          .map( |byte| byte.to_ascii_uppercase() )
          .collect::<Vec<u8>>()
      } );

      Ok( Response::new( Body::wrap_stream( mapping ) ) )
    },

    (&Method::POST, "/echo/reversed") => {
      let full_body = hyper::body::to_bytes( req.into_body() ).await?;
      // let reversed = full_body.reverse();
      let reversed = full_body.iter().rev().cloned().collect::<Vec<u8>>();

      Ok( Response::new( reversed.into() ) )
    }

    _ => {
      let mut response = Response::default();

      *response.status_mut() = StatusCode::NOT_FOUND;

      Ok( response )
    }
  }
}
async fn shutdown_signal() {
  tokio::signal::ctrl_c().await
    .expect( "failed to install CTRL+C signal handler" );
}

#[tokio::main]
async fn main() {
  let addr = ([127, 0, 0, 1], 80).into();

  let service = make_service_fn( |_| async { Ok::<_, hyper::Error>( service_fn( service ) ) } );

  let server = Server::bind( &addr ).serve( service );
  let server = server.with_graceful_shutdown( shutdown_signal() );

  println!( "\nListening on http://{}\n", addr );

  if let Err( e ) = server.await {
    eprintln!( "server error: {}", e );
  }
}