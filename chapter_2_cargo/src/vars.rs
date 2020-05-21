// Varaible hold primitive data types or refereces to data
// Variables are immutable
// Variables are block-scoped

pub fn run() {
  let name = "name";
  let mut age = 26;
  println!("Age was {}", age);
  age = 27;
  println!("My name is {} and age is {}", name, age);

  // Define constants
  const ID: i32 = 1;
  println!("ID is {}", ID);

  // Assign multiple variables
  let ( my_name, my_age ) = ("Adeel", 27);
  println!("Name {}, Age {}", my_name, my_age);
}