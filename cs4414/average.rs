use std::os;

fn average(numbers: &[~str]) -> float {
  let mut sum: float = 0.0;
  let mut count: float = 0.0;
  for number in numbers.iter().skip(1) {
    match from_str::<int>(*number) {
      Some(n) => {
        sum += (n as float);
        count += 1.0;
      }
      None => println!("Bad Input: {}", *number)
    }
  }
  sum / count
}

fn main() {
  let args: ~[~str] = os::args();
  let numbers = args;
  let avr = average(numbers);

  println!("Average: {}", avr)
}