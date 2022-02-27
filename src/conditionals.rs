// if - else basically
pub fn run() {
  let guess = "44".parse::<i32>().expect("Not a number!");
  let answer = 42;
  if guess == answer {
    println!("You got it!");
  } else if guess + 1 == answer || guess - 1 == answer {
    println!("You're close!");
  } else {
    println!("You're wrong!");
  }
  //inline if statement
  let is_even = if guess % 2 == 0 { true } else { false };
  println!("Is it even? {}", is_even);
}
