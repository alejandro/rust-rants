use std::path;
use std::io;

fn read_file(path: ~str) -> ~[~str]{
  let filereader : Result<@Reader, ~str> = io::file_reader(~path::Path(path));
  match filereader {
    Ok(reader) => reader.read_lines(),
    Err(msg) => fail!("Cannot open file: " + msg),
  }
}
fn main() {
  let path: ~[~str] = read_file(~"./str.txt");

  println(fmt!("%?", path))
  
}