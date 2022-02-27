// structure
struct Color {
  r: u8,
  g: u8,
  b: u8,
}

struct Person {
  name: String,
  age: u8,
}
impl Person {
  // constructor
  fn new(name: String, age: u8) -> Person {
    Person { name, age }
  }
  fn change_name(&mut self, new_name: &str) {
    self.name = new_name.to_string();
  }
  fn print(&self) {
    println!("{} is {} years old", self.name, self.age);
  }
}

// tuple struct
struct Point(i32, i32, i32);

pub fn run() {
  let mut red = Color { r: 255, g: 0, b: 0 };
  red.r = 200;
  println!("{:?}", (red.r, red.g, red.b));

  let mut person = Person::new(String::from("John"), 20);
  person.change_name("Jane");
  person.print();

  let mut point = Point(0, 0, 0);
  point.0 = 1;
  println!("{:?}", (point.0, point.1, point.2));
}
