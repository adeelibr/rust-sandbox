// enums are types which have definitive values

enum Movement {
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  // perform action depending on movement
  match m {
    Movement::Up => println!("Avatar moved up"),
    Movement::Down => println!("Avatar moved down"),
    Movement::Left => println!("Avatar moved left"),
    Movement::Right => println!("Avatar moved right")
  }
}

pub fn run() {
  let avatar1 = Movement::Left;
  let avatar2 = Movement::Right;
  let avatar3 = Movement::Up;
  let avatar4 = Movement::Down;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}