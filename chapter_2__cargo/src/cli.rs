use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let commands = args[1].clone();
  println!("Args {:?}", args);
  println!("Argument (1) {:?}", commands);
}