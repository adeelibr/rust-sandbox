// struct Color {
//   red: u8,
//   green: u8,
//   blue: u8,
// }

// // tuple struct
// struct TupleColor(u8, u8, u8);

// pub fn run() {
//   let mut c = Color {
//     red: 255,
//     green: 0,
//     blue: 0,
//   };
//   c.red = 200;
//   println!("{} {} {}", c.red, c.green, c.blue);

//   let mut c2 = TupleColor(0, 0, 0);
//   c2.0 = 200;
//   println!("{} {} {}", c2.0, c2.1, c2.2);
// }

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // constructor
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }
  // get full name
  fn get_full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }
  // set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }
  // name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut p = Person::new("John", "Doe");
  println!("Person is {} {}", p.first_name, p.last_name);
  p.set_last_name("Cena");
  println!("Person is {}", p.get_full_name());
  println!("Person tuple {:?}", p.to_tuple());
}