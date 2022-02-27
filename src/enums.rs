// enums are a way to represent a group of related values.
// enums are similar to structs, but they are limited to a specific set of values.

enum Movement {
  // Variants of an enum
  Up,
  Down,
  Left,
  Right,
}

fn move_character(m: Movement) {
  // match is a way to handle multiple cases
  match m {
    Movement::Up => println!("Move up"),
    Movement::Down => println!("Move down"),
    Movement::Left => println!("Move left"),
    Movement::Right => println!("Move Right"),
  }
}

pub fn run() {
  let character1 = Movement::Up;
  let character2 = Movement::Down;
  let character3 = Movement::Left;
  let character4 = Movement::Right;

  move_character(character1);
  move_character(character2);
  move_character(character3);
  move_character(character4);
}
