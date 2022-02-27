// arrays are a collection of values of the same type
// arrays are mutable
// arrays are fixed size
use std::mem;
pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{:?}", numbers);
  print!("first el: {}", numbers[0]);
  // length of array
  println!("{}", numbers.len());
  //memory address
  println!("{:p}", numbers.as_ptr());
  // size in bytes
  println!("{} bytes", mem::size_of_val(&numbers));
  //slice
  let slice: &[i32] = &numbers[1..4];
  println!("{:?}", slice);

  // loop through array
  for x in numbers.iter() {
    println!("number: {}", x);
  }

  // loop & mutate
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("{:?}", numbers);
}
