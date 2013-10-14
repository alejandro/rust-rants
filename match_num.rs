use std::io;
use std::rand;
mod extn;


fn main() {
  let num: int = (rand::random::<int>() % 100).abs() + 1;
  let mut i = 0;

  println(":: G U E S S   T H E   N U M B E R ::");
  println("(you have 5 tries):");

  loop {
    i += 1;
    let input = io::stdin().read_line();
    match from_str::<int>(input){
      Some(str_int) => { if extn::guess(str_int, num) { break; } }
      None => println("dude that's not a num")
    } 

    if i == 5 {
      println!("the number was {}", num);
      break;
    }
  }
}

