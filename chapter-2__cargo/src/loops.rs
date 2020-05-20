pub fn run() {
  let mut count = 0;

  // // infinite loop
  // loop {
  //   count += 1;
  //   println!("Number {}", count);
  // }

  // infinite loop with break statement
  // loop {
  //   count += 1;
  //   println!("Number {}", count);

  //   if count == 20 {
  //     break;
  //   }
  // }

  // while loop (fizzbuzz challenge)
  while count <= 100 {
    if count % 3 == 0 && count % 5 == 0 {
      println!("FizzBuzz at {}", count);
    } else if count % 3 == 0 {
      println!("Fizz at {}", count);
    } else if count % 5 == 0 {
      println!("Buzz at {}", count);
    } else {
      println!("{}", count);
    }
    count += 1;
  }

  // for range loop (fizzbuzz challenge)
  // for x in 0..100 {
  //   if x % 3 == 0 && x % 5 == 0 {
  //     println!("FizzBuzz at {}", x);
  //   } else if x % 3 == 0 {
  //     println!("Fizz at {}", x);
  //   } else if x % 5 == 0 {
  //     println!("Buzz at {}", x);
  //   } else {
  //     println!("{}", x);
  //   }
  // }  
}