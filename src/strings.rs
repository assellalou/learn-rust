//str -> strings are immutable fixed size
// String -> strings are mutable and growable and heap allocated

pub fn run() {
  // str
  let hello = "Hello";
  // String
  let mut world = String::from("World");
  // length
  println!("length is : {}", hello.len());
  //push char
  world.push(' ');
  // push string
  world.push_str("!!!!");
  // capacity (bytes)
  println!("capacity is : {}", world.capacity());
  // is empty
  println!("empty : {}", world.is_empty());
  // contains
  println!("contains 'World'?  {}", world.contains("World"));
  // replace
  println!("replace : {}", world.replace("World", "Love"));
  // loop through string by whitespace
  for word in world.split_whitespace() {
    println!("{}", word);
  }

  println!("{} {}", hello, world);

  // create string with capacity
  let mut s = String::with_capacity(20);
  s.push('a');
  s.push('s');
  s.push_str("sellalou");

  println!("{}", s);
  // assert equal
  assert_eq!(s, "assellalou");
  assert_ne!(10, s.capacity());

  // get slice
  let ass = &s[0..3];
  let lalo = &s[5..9];
  println!("{} {}", ass, lalo);
}
