use std::mem;

pub fn run() {
  let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
  // console all items
  println!("numbers: {:?}", numbers);
  // get single item
  println!("numbers[2]: {:?}", numbers[2]);
  
  // make vector mutatable
  let mut other: Vec<i32> = vec![1, 2, 3, 4, 5];
  // change vector value
  other[2] = 30;
  // push items to vector
  other.push(6);
  other.push(7);
  // pop last item
  other.pop();

  // get vector length
  println!("other vector length is {}", other.len());
  // vector are stack allocated (fied size)
  println!("other vector size is {} bytes", mem::size_of_val(&other));

  // loop through vector values
  for x in other.iter() {
    println!("Other: {}", x);
  }
  // loop through vector values and mutate
  for x in other.iter_mut() {
    *x *= 2;
  }
  println!("Other mutated: {:?}", other);

  // get slice
  let slice: &[i32] = &numbers;
  println!("Slice is {:?}", slice);
  // get slice (in range)
  let slice: &[i32] = &numbers[0..2];
  println!("Slice is {:?}", slice);
}