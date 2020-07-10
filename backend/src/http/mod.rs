mod mime_types;
mod service;

use std::net::SocketAddr;
use hyper::Server as HyperServer;
use hyper::service::{ make_service_fn, service_fn };

async fn shutdown_signal() {
  tokio::signal::ctrl_c().await
    .expect( "failed to install CTRL+C signal handler" );
}

pub async fn run( addr:SocketAddr ) {
  let service = make_service_fn(|_| async {
    Ok::<_, hyper::Error>( service_fn( service::service ) )
  } );

  let server = HyperServer::bind( &addr ).serve( service );
  let server = server.with_graceful_shutdown( shutdown_signal() );

  println!( "HTTP server ready on http://{}", addr );

  if let Err( e ) = server.await {
    eprintln!( "server error: {}", e );
  }
}