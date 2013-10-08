fn main() {
  for num in range(1, 101)  {
    println(fizzbuzz(num))
  }
}

fn fizzbuzz(num: int) -> ~str {
  match (es_tres(num), es_cinco(num)){
    (true, true) => ~"FizzBuzz",
    (true, false) => ~"Fizz",
    (false, true) => ~"Buzz",
    (false, false) => num.to_str()
  } 
}

fn es_tres(num: int) -> bool {
  num % 3 == 0
}

fn es_cinco(num: int) -> bool {
  num % 5 == 0
}

#[test]
fn checkFizz(){
  assert!(fizzbuzz(3) == ~"Fizz")
  assert!(fizzbuzz(5) == ~"Buzz")
  assert!(fizzbuzz(15) == ~"FizzBuzz")
  assert!(fizzbuzz(7) == ~"7")
}

#[test]
fn checkDividers(){
  assert!(es_tres(3))
  assert!(es_cinco(5))
}
