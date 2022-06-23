# Kaprekar's Constant Counter

Kaprekar's constant (6174) is useless but fun mathemtatical phenomenon.

## Quick Start

The function within the crate you want is `kaprekar::calculate(XXXX)` which takes a four-digit number with different integers.

E.g.

```
use kaprekar;

  ...
  let answer = kaprekar::calculate(1234);
  // answer == 3
  ...

```

## How it Works

1. Take any four-digit number where all four digits aren't the same (e.g. 1832 or 9015, not 4444 of 0000)
2. Arrange the digits into ascending and descending order (8321 and 1238) and sum
3. Repeat the process with the sum (and each successive sum) until it reaces 6174

Applying this process to 6174 will result in 6174.

This function accepts an input of a four-digit, non-repeating number and returns the number of iterations to reach 6174.

If you submit an invalid integer (either repeating or greater than 4 digits) it will return an error.

- Validate the input (AKA a four-digit number with differing integers)
- Instantiate counter variable

And then invoke a recursive function that performs this calculation:

1. Convert the integer -> string
2. Split string -> vector of characters
3. Reorganize vector into two different ones, ascending and descending order, e.g. `4123` -> `["4","3","2","1"]` and `["1","2","3","4"]`
4. Convert both vectors strings again
5. And convert strings back to into integers
6. Return the difference of the two integers
7. If the answer isn't 6174, increment the counter and run it all again

### Return Codes

- Any positive integer is the number of iterations the formula took.
- A return value of _-1_ means the input was out of bounds
- A return value of 0 means the input itself was 6174.
- Any other positive integer is the number of iterations it took to reach the final result of 6174.
