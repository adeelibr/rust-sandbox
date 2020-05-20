/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run() {
  // default i32
  let i = 1;
  // default f64
  let f = 2;
  // add type manually
  let foo: i64 = 25004390490329;
  // console to screen
  println!("{}, {}, {}", i, f, foo);
  // find max
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);
  // boolean
  let is_fat: bool = true;
  // boolean from expression
  let is_greater_then_10 = 11 > 10;
  println!("{:?}", (is_fat, is_greater_then_10));
  // chars
  let a1: char = 'a';
  let face: char = '\u{1F600}';
  println!{"{:?}", (a1, face)}; 
}