Kaprekar's Constant Counter

Kaprekar's constant (6174) is useless but fun mathemtatical phenomenon.

1. Take any four-digit number where all four digits aren't the same (e.g. 1832 or 9015, not 4444 of 0000)
2. Arrange the digits into ascending and descending order (8321 and 1238) and sum
3. Repeat the process with the sum (and each successive sum) until it reaces 6174

Applying this process to 6174 will result in 6174.

This function accepts an input of a four-digit, non-repeating number and returns the number of iterations to reach 6174.

If you submit an invalid integer (either repeating or greater than 4 digits) it will return an error.

- validate the input (AKA four numbers that aren’t all the same digit)
- instantiate counter variable
- start a loop, run this function:
  - split and organize numbers
    - integer -> to string
    - string - split into vector
    - reorganize vector (forward and backward)
    - convert both vectors strings to into integer again
    - return a vector of both numbers
  - add the two numbers
    - if they don’t add up to 6174, run the loop again
    - if they do, break return counter (breaking the loop and exiting the function)

### Return Codes

- Any positive integer is the number of iterations the formula took.
- A return value of _-1_ means the input was out of bounds
- A return value of 0 means the input itself was 6174.
- Any other positive integer is the number of iterations it took to reach the final result of 6174.
