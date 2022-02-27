use ferris_says::say;
use std::io::{stdout, BufWriter};
// mod print;
// mod vars;
// mod types;
// mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointers_ref;
// mod structures;
// mod enums;
// mod cli;

fn main() {
  let stdout = stdout();
  let message = String::from("Hello rust!");
  let width = message.chars().count();
  let mut writer = BufWriter::new(stdout.lock());
  say(message.as_bytes(), width, &mut writer).unwrap();

  // print::run();
  // vars::run();
  // types::run();
  // strings::run();
  // tuples::run();
  // arrays::run();
  // vectors::run();
  // conditionals::run();
  // loops::run();
  // functions::run();
  // pointers_ref::run();
  // structures::run();
  // enums::run();
  // cli::run();
}
