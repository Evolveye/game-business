mod service;

use hyper::Server;
use hyper::service::{ make_service_fn, service_fn };

async fn shutdown_signal() {
  tokio::signal::ctrl_c().await
    .expect( "failed to install CTRL+C signal handler" );
}

#[tokio::main]
async fn main() {
  let addr = ([127, 0, 0, 1], 80).into();

  let service = make_service_fn(|_| async {
    Ok::<_, hyper::Error>( service_fn( service::service ) )
  } );

  let server = Server::bind( &addr ).serve( service );
  let server = server.with_graceful_shutdown( shutdown_signal() );

  println!( "\nListening on http://{}\n", addr );

  if let Err( e ) = server.await {
    eprintln!( "server error: {}", e );
  }
}