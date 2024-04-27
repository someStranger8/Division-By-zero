
/*
  division by zero
*/

import java.util.Math;

public class Main {
    public static void Main(String[] args) {
        // division by zero
        System.out.println(div(0,0));
    }

    public static string div(int a, int b) {
      // count how many times b is in a
      int i = 0;

      // loop through it
      while (a >= b) {
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
}
