use std::io;
use std::rand;

fn main() {
  let num: int = (rand::random::<int>() % 100).abs() + 1;
  let mut i = 0;

  println(":: G U E S S   T H E   N U M B E R ::");
  println("(you have 5 tries):");

  loop {
    i += 1;
    let input = io::stdin().read_line();
    match from_str::<int>(input){
      Some(str_int) => {
        if user_guessed(str_int, num) { break;}
      }
      None => {
        println("dude that's not a num");
      }
    } 

    if i == 5 {
      println(fmt!("the number was %d", num));
      break;
    }
  }
}

fn user_guessed(val: int, num: int) -> bool{
 if val == num {
    println(fmt!("You guessed the number:: %d", val));
    true
  } else if val < num { 
    println("the number is a little bit higher");
    false
  } else {
    println("the number is lower");
    false
  }
}