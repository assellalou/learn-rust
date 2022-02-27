//vectors are a collection of values of the same type
//vectors are mutable
//vectors are growable (can add elements)

use std::mem;
pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
  println!("{:?}", numbers);
  // add element
  numbers.push(6);
  numbers.push(7);
  println!("added 2 els: {:?}", numbers);
  //remove element
  numbers.pop();
  println!("removed last el: {:?}", numbers);
  // get single value
  println!("first item: {}", numbers[0]);
  // length of vector
  println!("length: {}", numbers.len());
  //memory address
  println!("address: {:p}", numbers.as_ptr());
  // size in bytes
  println!("size: {} bytes", mem::size_of_val(&numbers));
  //slice
  let slice: &[i32] = &numbers[1..4];
  println!("slice: {:?}", slice);
  // loop through vector
  for x in numbers.iter() {
    println!("number: {}", x);
  }
  // loop & mutate
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("multiplied by 2: {:?}", numbers);
}
