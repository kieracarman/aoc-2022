use template::{a, b};
use std::fs;

fn main() {
  let file = fs::read_to_string("input.txt").unwrap();
  println!("Part 1: {}\nPart 2: {}", a(&file), b(&file));
}