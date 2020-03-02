use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  let secret = rand::thread_rng().gen_range(1, 101);

  loop {
    println!( "Give me a number!" );

    let mut guess = String::new();

    io::stdin().read_line( &mut guess )
      .expect( "We can't read the line" );

    let guess: i32 = match guess.trim().parse() {
      Ok( num ) => num,
      Err(_) => continue,
    };

    match guess.cmp( &secret ) {
      Ordering::Greater => println!( "Too big" ),
      Ordering::Less => println!( "Too small" ),
      Ordering::Equal => {
        println!( "You win!" );
        break;
      },
    }
  }
}
