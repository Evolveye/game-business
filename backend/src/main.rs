mod http;
mod ws;

// use futures::executor::block_on;

#[tokio::main]
async fn main() {
  println!( "\n" );

  futures::join!(
    http::run( ([127, 0, 0, 1], 80).into() ),
    ws::run( ([127, 0, 0, 1], 8080).into() ),
  );

  todo!();
}