# Kaprekar's Constant Counter

Kaprekar's constants (6174 & 495) is useless but fun mathemtatical phenomenon.

## Quick Start

The function within the crate you want is `kaprekar::calculate_four(XXXX)` which takes a four-digit number with different integers.

There is also a three-digit constant (495) which you can calculate by inputing a three-digit input with `kaprekar::calculate_three(XXX)`.

E.g.

```
use kaprekar;

  ...
  let answer = kaprekar::calculate_four(1234);
  // answer == 3
  ...

```

## How it Works

1. Take any four-digit number where all four digits aren't the same (e.g. 1832 or 9015, not 4444 of 0000)
2. Arrange the digits into ascending and descending order (8321 and 1238) and sum
3. Repeat the process with the sum (and each successive sum) until it reaces 6174

Applying this process to 6174 will result in 6174.

This function accepts an input of a four-digit, non-repeating number and returns the number of iterations to reach 6174.

#### This same principle applies to three digit numbers.

E.g. supplying 993 will go through the same process, except the converging number is 495.

If you submit an invalid integer (either repeating or greater than 4 digits for `calculate_four` or greater than 3 digits for `calculate_three`) it will return an error.

## How this library works

1. Validate the input (AKA correct number of differing integers)
2. Instantiate counter variable
3. Invoke a recursive function that performs this calculation:
   a. Convert the integer -> string
   b. Split string -> vector of characters
   c. Reorganize vector into two different ones, ascending and descending order, e.g. `4123` -> `["4","3","2","1"]` and `["1","2","3","4"]`
   d. Convert both vectors into separate strings again
   e. And convert strings back to into separate integers
   f. Return the difference of the two integers
   g. If the answer isn't 6174, increment the counter and run it all again from the top of step 3

### Return Codes

- Any `Some(positive_integer)` is the number of iterations the formula took.
- A return value of `Some(0)` means the given input itself was 6174 or 495 (hence `0` manipulations needed).
- A return value of `None` means the input was out of bounds or was all repeating digits
