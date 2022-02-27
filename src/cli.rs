// cli arguments are passed in as a vector of strings.
// The first element is the name of the program.
// The rest of the elements are the arguments.
pub fn run() {
  let args: Vec<String> = std::env::args().collect();
  let command = args[1].clone();
  println!("first arguemnt: {}", command);
}
