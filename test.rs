fn main() {
  let a = 2;
  println(fmt!("%?",  addHola(a)))
}

fn addHola(wrd: int) -> int {
  wrd + 3
}