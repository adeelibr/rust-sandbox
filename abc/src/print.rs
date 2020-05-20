//  Print to console
pub fn run() {
  // basic formatting
  println!("Hello, world!");
  println!("{}", 1);
  println!("This is how numbers work {} in string", 3000);
  println!("{} is learning {}", "Adeel", "rust");
  // postional formatting
  println!("{0} is learning {1} and {0} is {2} it", "Adeel", "rust", "loving");
  // named arguments
  println!("{name} is learning {activity}", name = "Adeel", activity = "rust");
  // placeholder traits
  println!("Binary {:b}, Hex {:x}, Octal {:o}", 10, 10, 10);
  // placeholder for debug trait
  println!("{:?}", (10, "hello", "foo"));
  // expressions
  println!("10 + 10 = {}", 10 + 10);
}