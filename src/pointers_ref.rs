// Reference pointer is a reference to a value that is stored in memory.
// Passing by ref is a way to pass a value, by passing it's address rather than the value itself.
pub fn run() {
  let arr1 = [1, 2, 3, 4, 5];
  let arr2 = arr1;
  println!("addr of arr1: {:?}", arr1.as_ptr());
  println!("addr of arr2: {:?}", arr2.as_ptr());
  println!("Values: {:?}", (arr1, arr2));

  let vec1 = vec![1, 2, 3, 4, 5];
  let vec2 = &vec1;
  println!("addr of vec1: {:?}", vec1.as_ptr());
  println!("addr of vec2: {:?}", vec2.as_ptr());
  println!("Values: {:?}", (&vec1, vec2));
}
