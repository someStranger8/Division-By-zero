
/*
  division by zero
*/

#include <iostream>
using namespace std;

char div(int a, int b) {
  // count how many times b is in a
  int i = 0;

  // loop through it
  while (a >= b) {
    // without remainder
    if (a == b) {
      int l = i + 1;
      return "" + l;
    }
    
    // keep counting
    int i = i + 1;
    int a = a - b;
  }

  // with remainder
  return "" + i + " with a remainder of " + a;
}

int main() {
  // division by zero
  cout << div(0,0) << endl;
  return 0;
}
