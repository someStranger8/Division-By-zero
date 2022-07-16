
/*
  division by zero
*/

fun div(a: int, b: int) {
  // count how many times b is in a
  var i = 0

  // loop through it
  while (a >= b) {
    // without remainder
    if (a == b) {
      var l = i + 1
      return l
    }
    
    // keep counting
    var i = i + 1
    var a = a - b
  }

  // with remainder
  return i + " with a remainder of " + a
}

fun main() { 
  // division by zero
  println(div(0,0)) 
}
