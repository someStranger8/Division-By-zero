<?php

/*
  division by zero
*/

function div($a, $b) {
  // count how many times a is in b
  $i = 0;
  
  // loop through it
  while ($a >= $b) {
    if ($a == $b) {
      // without remainder
      return $i + 1;
    }
    
    // keep counting
    $i = $i + 1;
    $a = $a - $b;
  }
  
  // with remainder
  return $i + " with a remainder of " + $a;
}

echo div(0,0);
?>
