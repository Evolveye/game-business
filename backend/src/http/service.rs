use super::mime_types::get_mime_type;
use std::fs;
use hyper::{ Method, StatusCode, Body, Request, Response };

pub const FRONTEND_BUILD_PATH: &str = "../frontend/build/";

pub async fn service( req:Request<Body> ) -> Result<Response<Body>, hyper::Error> {
  let uri_path = req.uri().path();
  let mime_type = get_mime_type( uri_path );

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

    _ => {
      let mut response = Response::default();

      *response.status_mut() = StatusCode::NOT_FOUND;

      Ok( response )
    }
  }
}