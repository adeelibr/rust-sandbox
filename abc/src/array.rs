// array in rust are fixed length elements with the same data type

use std::mem;

pub fn run() {
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];
  // console all items
  println!("numbers: {:?}", numbers);
  // get single item
  println!("numbers[2]: {:?}", numbers[2]);
  
  // make array mutatable
  let mut other: [i32; 5] = [1, 2, 3, 4, 5];
  println!("(before) other: {:?}", other);
  // change array value
  other[2] = 30;
  println!("(after) other: {:?}", other);

  // get array length
  println!("other array length is {}", other.len());
  // array are stack allocated (fied size)
  println!("other array size is {} bytes", mem::size_of_val(&other));

  // get slice
  let slice: &[i32] = &numbers;
  println!("Slice is {:?}", slice);
  
  // get slice (in range)
  let slice: &[i32] = &numbers[0..2];
  println!("Slice is {:?}", slice);
}