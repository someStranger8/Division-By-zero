
/*
  division by zero
*/

using System;

namespace Hello {
    class Program {
        static void Main(string[] args) {
            Console.WriteLine(div(0,0));
        }

        static void div(int a, int b) {
            // count how many times b is in a
            int i = 0;

            // loop through it
            while (a > b) {
                // without remainder
                if (a == b) {
                  return i + 1;
                }
                
                // keep counting
                a -= b;
                i++;
            }

            // with remainder
            return i + " with a remainder of " + a;
        }
    }
}
