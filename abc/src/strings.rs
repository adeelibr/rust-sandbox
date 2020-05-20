// Primitive string = immutable string (fixed length in memory)
// String = growable heap allocated data structure - use when you need to modify string

pub fn run() {
  let mut hello = String::from("Hello");
  println!("{}", hello);
  // Get length
  println!("length of of var hello :: {}", hello.len());
  // push single character
  hello.push(' ');
  hello.push('W');
  // push string
  hello.push_str("orld!");
  println!("{}", hello);
  // capacity
  println!("Capacity {}", hello.capacity());
  // check is string empty
  println!("Is hello var empty :: {}", hello.is_empty());
  // check is string empty
  println!("Contains `World` string in hello var :: {}", hello.contains("World"));
  // check is string empty
  println!("Replace :: {}", hello.replace("World", "Mars"));


  println!("------ Start of for loop -----");
  // loop through string with whitespace
  for word in hello.split_whitespace() {
    println!("Word is :: {}", word);
  }
  println!("------ End of for loop -----");

  // create string with capacity
  let mut cap = String::with_capacity(4);
  cap.push('a');
  cap.push('b');
  println!("{}", cap);
  println!("{}", cap.capacity());

  // assertion testing (will only log something if it fails)
  assert_eq!(2, cap.len());
  assert_eq!(4, cap.capacity());
  
}