mod mime_types;

use std::fs;
use hyper::{ Method, StatusCode, Body, Request, Response };
use futures::TryStreamExt as _;

pub const FRONTEND_BUILD_PATH: &str = "../frontend/build/";

pub async fn service( req:Request<Body> ) -> Result<Response<Body>, hyper::Error> {
  let uri_path = req.uri().path();
  let mime_type = mime_types::get_mime_type( uri_path );

  match (req.method(), uri_path) {
    (&Method::GET, _) => {
      let mut res = Response::builder();
      let path;

      if mime_type == "" {
        path = format!( "{}/index.html", FRONTEND_BUILD_PATH );
        res = res.header( "Content-Type", "text/html" );
      } else {
        path = format!( "{}{}", FRONTEND_BUILD_PATH, uri_path );
        res = res.header( "Content-Type", mime_type );
      }

      println!( "uri: {: <50} mime: {: <20} path: {:?}", uri_path, mime_type, path );

      let file = fs::read( path ).unwrap();
      let res = res.body( Body::from( file ) ).unwrap();

      Ok( res )
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