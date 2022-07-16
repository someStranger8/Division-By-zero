
/*
  division by zero
*/

package main
import ("fmt")

func div(a, b) {
  // count how many times b is in a
  var i = 0;

  // loop through it
  for a > b {
    // without remainder
    if a == b {
      var l = i + 1;
      return l;
    }
    
    // keep counting
    var i = i + 1;
    var a = a - b;
  }

  // with remainder
  return "" + i + " with a remainder of " + a;
}

func main() {
  // division by zero
  var y = 0/0
  fmt.Println(y)
}
