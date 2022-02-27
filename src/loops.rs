pub fn run() {
  let mut i = 0;

  // infinite loop
  loop {
    println!("Number: {}", i);
    i += 1;
    if i > 10 {
      break;
    };
  }
  // while loop
  while i <= 100 {
    if i % 15 == 0 {
      println!("FizzBuzz");
    } else if i % 3 == 0 {
      println!("Fizz");
    } else if i % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", i);
    }
    i += 1;
  }

  // for range loop
  for i in 0..100 {
    if i % 15 == 0 {
      println!("FizzBuzz");
    } else if i % 3 == 0 {
      println!("Fizz");
    } else if i % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", i);
    }
  }
}
