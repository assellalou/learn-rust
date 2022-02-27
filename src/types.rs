// basic types
// bool - boolean
// char - character
// i8, i16, i32, i64, isize - signed integer
// u8, u16, u32, u64, usize - unsigned integer
// f32, f64 - floating point
// tuple - fixed length tuple
// array - fixed length array
// rust is statically typed language - types are defined when compiling the code and cannot be changed later,
// however, the compiler can infer the type of variables based on the value assigned to them
pub fn run() {
  // default is "i32"
  let x = -200;
  // default is "f64"
  let y = 3.14;
  // add explicit type
  let z: i64 = 4500000;
  // find maximum value
  println!("max i32: {}", std::i32::MAX);
  println!("max i64: {}", std::i64::MAX);

  //bool
  let is_true = false;
  // char
  let a = 'a';
  let face = '\u{1F4A9}';
  println!("{:?}", (x, y, z, is_true, a, face));
}
