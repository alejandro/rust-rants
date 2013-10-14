use std::os;

fn main() {
  let args: ~[~str] = os::args();
  let valids = args.slice(1, args.len());

  for x in valids.iter() {
    print(fmt!("%s ", *x));
  }
  print("\n");
}