// use super::mime_types::get_mime_type;
// use std::fs;

// use tokio::io::{ AsyncReadExt, AsyncWriteExt };
// use futures::prelude::*;
// use futures::stream::StreamExt;
// use hyper::{
//   header::{ AsHeaderName, HeaderMap, HeaderValue, UPGRADE, SEC_WEBSOCKET_KEY, SEC_WEBSOCKET_VERSION, SEC_WEBSOCKET_ACCEPT },
//   upgrade::OnUpgrade,
//   Method, StatusCode, Body, Request, Response
// };
// use std::io;
// use tokio_tungstenite::{
//   tungstenite::protocol::{ Message, Role },
//   accept_async,
//   WebSocketStream
// };
// use sha1::{ Digest, Sha1 };

// pub const FRONTEND_BUILD_PATH: &str = "../frontend/build/";

// // fn ws(ws: WebSocketStream<impl AsyncRead + AsyncWrite>) -> impl Future<Item = (), Error = ()> {
// //   let (sink, stream) = ws.split();
// //   let responses = stream.filter_map(|message| {
// //       println!("message: {}", message);
// //       match message {
// //           Message::Text(text) => Some(Message::text("echo: ".to_string() + &text)),
// //           Message::Binary(_) => Some(Message::text("no binary!")),
// //           Message::Ping(data) => Some(Message::Pong(data)),
// //           Message::Pong(_) => None,
// //           Message::Close(_) => None,
// //       }
// //   });
// //   sink.send_all(responses).map(|_| ()).map_err(|e| {
// //       eprintln!("failed websocket echo: {}", e);
// //   })
// // }
// fn header_matches<S: AsHeaderName>(headers: &HeaderMap<HeaderValue>, name: S, value: &str) -> bool {
//   headers
//       .get(name)
//       .and_then(|v| v.to_str().ok())
//       .map(|v| v.to_lowercase() == value)
//       .unwrap_or(false)
// }
// fn handle_ws_connection(req:Request<Body>) -> Result<Response<Body>, io::Error> {
//   let res = match upgrade_connection(req) {
//       Err(res) => res,
//       Ok((res, ws)) => {
//           let run_ws_task = async {
//               match ws.await {
//                   Ok(ws) => {
//                       println!("Spawning WS");
//                       let mut counter: i32 = 0;
//                       let (tx, rc) = ws.split();
//                       let rc = rc.try_filter_map(|m| {
//                           println!("Got message {:?}", m);
//                           future::ok(match m {
//                               Message::Text(text) => {
//                                   counter += 1;
//                                   Some(Message::text(format!(
//                                       "Response {}: {}",
//                                       counter, text
//                                   )))
//                               }
//                               _ => None,
//                           })
//                       });
//                       match rc.forward(tx).await {
//                           Err(e) => eprintln!("WS Error {}", e),
//                           Ok(_) => println!("Websocket has ended"),
//                       }
//                   }
//                   Err(_e) => eprintln!("WS error"),
//               }
//           };
//           tokio::spawn(run_ws_task);
//           res
//       }
//   };
//   println!("WS HTTP Response {:?}", res);
//   Ok(res)
// }
// fn upgrade_connection( req:Request<Body> ) -> Result<
//   (Response<Body>, Result<impl Future<Output = WebSocketStream<OnUpgrade>>, ()>),
//   Response<Body>
// > {
//   let mut res = Response::new(Body::empty());
//   let mut header_error = false;
//   println!("We got these headers: {:?}", req.headers());

//   if !header_matches(req.headers(), UPGRADE, "websocket") {
//     eprintln!("Upgrade is not to websocket");
//     header_error = true;
//   }

//   if !header_matches(req.headers(), SEC_WEBSOCKET_VERSION, "13") {
//     eprintln!("Websocket protocol version must be 13");
//     header_error = true;
//   }

//   if !req.headers().contains_key( UPGRADE ) {
//     eprintln!( "It must be upgrade connection" );
//     header_error = true;
//   }

//   // if key.is_none() {
//   //   eprintln!("Websocket key missing");
//   //     header_error = true;
//   // }

//   if header_error {
//       *res.status_mut() = StatusCode::BAD_REQUEST;
//       return Err(res);
//   }

//   let key = req.headers().get( SEC_WEBSOCKET_KEY ).unwrap();
//   let accept = {
//     let hasher = Sha1::new();

//     hasher.update( String::from( key.to_str().unwrap() ) );
//     hasher.update( "258EAFA5-E914-47DA-95CA-C5AB0DC85B11" );

//     base64::encode( hasher.finalize() )
//   };

//   *res.status_mut() = StatusCode::SWITCHING_PROTOCOLS;
//   res.headers_mut().insert( UPGRADE, HeaderValue::from_static( "webSocket" ) );
//   res.headers_mut().insert( SEC_WEBSOCKET_ACCEPT, HeaderValue::from_static( &accept ) );
//   // h.typed_insert(headers::Connection::upgrade());

//   let upgraded = req
//       .into_body()
//       .on_upgrade()
//       .map_err(|err| eprintln!("Cannot create websocket: {} ", err))
//       .and_then(|upgraded| async {
//           println!("Connection upgraded to websocket");
//           let r = WebSocketStream::from_raw_socket(upgraded, Role::Server, None);

//           Ok(r)
//       });

//   // let wsStream = WebSocketStream::from_raw_socket(upgraded, Role::Server, None);

//   Ok((res, upgraded))
// }

// pub async fn service( req:Request<Body> ) -> Result<Response<Body>, hyper::Error> {
//   let uri_path = req.uri().path();
//   let mime_type = get_mime_type( uri_path );

//   if req.headers().contains_key( UPGRADE ) {
//     match upgrade_connection( req ) {
//       Ok(( res, upgraded )) => {

//         // accept_async( upgraded );
//         // let upgraded = req.into_body()
//         //   .on_upgrade()
//         //   .map_err(|err| eprintln!("Cannot create websocket: {} ", err))
//         //   .and_then(|upgraded| async {
//         //     println!("Connection upgraded to websocket");
//         //     let ws = WebSocketStream::from_raw_socket(upgraded, Role::Server, None);

//         //     accept_async( ws );

//         //     Ok( ws )
//         //   });

//         // accept_async( upgraded );//.and_then();
//         // tokio::spawn( upgraded.and_then( ws )  );
//       }

//       Err( res ) => {

//       }
//     }

//     tokio::task::spawn( async {
//       match req.into_body().on_upgrade().await {
//         Ok(mut upgraded) => {
//           // let mut vec = Vec::new();
//           println!( "WS upgrade" );
//           // upgraded.read_to_end(&mut vec).await.unwrap();
//           upgraded.write_all( b"aaa" ).await.unwrap();
//           println!( "after read" );
//         }
//         Err(e) => eprintln!("upgrade error: {}", e),
//       }
//     } );

//     let mut response = Response::new( Body::empty() );

//     *response.status_mut() = StatusCode::SWITCHING_PROTOCOLS;
//     response.headers_mut().insert( UPGRADE, HeaderValue::from_static( "webSocket" ) );

//     Ok( response )
//   } else {
//     match (req.method(), uri_path) {
//       (&Method::GET, _) => {
//         let mut res = Response::builder();
//         let path;

//         if mime_type == "" {
//           path = format!( "{}/index.html", FRONTEND_BUILD_PATH );
//           res = res.header( "Content-Type", "text/html" );
//         } else {
//           path = format!( "{}{}", FRONTEND_BUILD_PATH, uri_path );
//           res = res.header( "Content-Type", mime_type );
//         }

//         println!( "uri: {: <50} mime: {: <20} path: {:?}", uri_path, mime_type, path );

//         let file = fs::read( path ).unwrap();
//         let res = res.body( Body::from( file ) ).unwrap();

//         Ok( res )
//       },

//       _ => {
//         let mut response = Response::default();

//         *response.status_mut() = StatusCode::NOT_FOUND;

//         Ok( response )
//       }
//     }
//   }
// }