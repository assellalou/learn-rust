// tuples are a collection of values of different types
// tuples are immutable
// tuples are fixed size (max 12 elements)
// tuples are stack allocated
// tuples can be destructured

pub fn run() {
  // tuple creation
  let me: (&str, &str, i8) = ("Assellalou", "Morocco", 22);
  println!("{} is from {} and is {}", me.0, me.1, me.2);

  // destructuring
  let (my_name, my_country, my_age) = me;
  println!("{} is from {} and is {}", my_name, my_country, my_age);

  // tuples can be destructured to create variables
  let my_name = me.0;
  let my_country = me.1;
  let my_age = me.2;
  println!("{} is from {} and is {}", my_name, my_country, my_age);
}
