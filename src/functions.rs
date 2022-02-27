pub fn run() {
  // print func returned value
  println!("{}", greeting("Rust"));
  // bind func to variable
  let get_mult = multiply(3.14, 5.0);
  println!("{}", get_mult);
  // closure
  let addition = |x: i32, y: i32| x + y;
  println!("{}", addition(1, 2));
}
fn greeting(name: &str) -> String {
  // return a string (no semi-colon)
  format!("Hello {}!", name)
}

fn multiply(x: f64, y: f64) -> f64 {
  x * y
}
