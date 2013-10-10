fn main() {
  let vec = [1,2,3];
  prt_vctr(vec);
  let str_vec = [~"ho", ~"la"];
  prt_vctr(str_vec);
}


fn prt_vctr<T: ToStr>(arr: &[T]){
  for k in arr.iter(){
    println((*k).to_str())
  }
}