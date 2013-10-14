// guessing "algorithm"
pub fn guess(val: int, num: int) -> bool{
 if val == num {
    println!("You guessed the number:: {}", val);
    true
  } else if val < num { 
    println("Wrong! The number is higher:");
    false
  } else {
    println("Wrong! The number is lower:");
    false
  }
}