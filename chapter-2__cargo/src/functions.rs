pub fn run() {
  greeting("Mahalo", "Batman");
  let get_sum = add(10, 5);
  println!("Sum is {}", get_sum);

  // closure
  let new_num: i32 = 5;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + new_num;
  println!("Closure sum {}", add_nums(2, 3));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}