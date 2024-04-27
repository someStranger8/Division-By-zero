
/*
  division by zero
*/

use std::fmt;

fn div(mut a, b) -> char {
  // count how many times b is in a
  let mut i = 0;
  
  // loop through it
  while (a >= b) {
    // return without remainder
    if (a == b) {
      format("{}", i + 1);
    }
    
    // keep counting
    let mut i = i + 1;
    let mut a = a - b;
  }
  
  // return with remainder
  format("{} with a remainder of {}", i, a);
}

fn main() {
  println(div(0,0));
}
