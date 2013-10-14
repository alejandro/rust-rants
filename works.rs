fn main(){
  let word = ~"Hi I'm am ";
  let sliced = slice(word, 3);
  println!("{} - {}", word, sliced);
}
 
fn slice<'a>(word: &'a str, size: uint) -> &'a str{
  word.slice_chars(0, size)
}