
/*
  division by zero
*/

function div(a, b) {
  // count how many times a is in b
  var i = 0;
  
  // loop through it
  while (a >= b) {
    if (a == b) {
      // without remainder
      return i + 1;
    }
    
    // keep counting
    i+=1;
    a-=b;
  }
  
  // with remainder
  return i + " with a remainder of " + a;
}

console.log(div(0,0));
