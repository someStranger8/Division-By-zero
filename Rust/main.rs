
/*
  division by zero
*/

use std::fmt;

fn div(mut a, b) -> char {
  // count how many times b is in a
  let mut i = 0;
  
  // loop through it
  while (a >= b) {
    // without remainder
    if (a == b) {
      let r = format("{}", i + 1);
      r
    }
    
    // keep counting
    let mut i = i + 1;
    let mut a = a - b;
  }
  
  // with remainder
  let l = format("{} with a remainder of {}", i, a);
  l
}

fn main() {
  println(div(0,0));
}
