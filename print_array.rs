fn main() {
  let vec = [1,2,3];
  let str_vec = [~"ho", ~"la"];
  prt_vctr(vec);
  prt_vctr(str_vec);
}


fn prt_vctr<T: ToStr>(arr: &[T]){
  for key in arr.iter(){
    println((*key).to_str())
  }
}

