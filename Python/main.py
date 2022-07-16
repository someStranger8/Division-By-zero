
"""
  division by zero
"""

def div(a, b):
  # count how many times b is in a
  i = 0

  # loop through it
  while a > b:
    # without remainder
    if a == 0:
      return i
    
    # keep counting
    i += 1
    a -= b
  
  # with remainder
  return f"{i} with a remainder of {a}"

print(div(0,0))
