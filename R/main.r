
# division by zero

div <- function(a, b) {
  # count how many times b is in a
  i <- 0

  # loop through it
  while (a >= b) {
    # without remainder
    if (a == b) {
      return i + 1
    }
    
    # keep counting
    i <- i + 1
    a <- a - b
  }

  # with remainder
  return i + " with a remainder of " + a
}

print(div(0,0))
