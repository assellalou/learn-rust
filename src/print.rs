pub fn run() {
  //basic formatting
  println!("Hello, rust! {}", 1999);
  //positional arguments
  println!("{0} {1} {0}", "Hello", "rust");
  //named arguments
  println!("{greeting}, {name}!", greeting = "Hello", name = "rust");
  //placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
  //placeholder for debug trait
  println!("{:?}", (12, true, "hello"));
  //placeholder for arrays
  let a = [1, 2, 3, 4, 5];
  println!("{:?}", a);
  //placeholder for vectors
  let v = vec![1, 2, 3, 4, 5];
  println!("{:?}", v);
  //placeholder for tuples
  let t = ("hello", 'c', 10);
  println!("{:?}", t);
  //placeholder for ranges
  for x in 0..5 {
    println!("{}", x);
  }
  //basic math
  println!("10 + 10 = {}", 10 + 10);
}
