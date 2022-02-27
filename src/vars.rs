// variables hold values or references to values
// variables are immutable by default (cannot reassign)
// variables can be mutable by using the mut keyword
// variables can be shadowed by using the same name in a different scope
pub fn run() {
  //immutable
  let name = "Assellalou";
  //constant
  const BIRTH_YEAR: i64 = 1999;
  //mutable
  let mut age = 20;

  println!(
    "Hello, I am {} and I was born in {} so my age is {}",
    name, BIRTH_YEAR, age
  );
  age = 22;
  println!(
    "Hello, I am {} and I was born in {} so my age is {}",
    name, BIRTH_YEAR, age
  );
  //multiple variables
  let (my_name, my_age) = ("Assellalou", 22);
  println!("{} is {}", my_name, my_age);
}
