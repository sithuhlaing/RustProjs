use std::util::Vector;


fn main() {

  let hm = Vector::new();

  hm.insert(3, "Hello");
  hm.insert(5, "world");

  let r = hm.get(&4).unwrap_or(&"NoString");

  println!("{}", r);


}
