fn parse_and_iterate(loop_counter: i16, number: i16) -> i16 {
    let int_to_str: String = number.to_string();
    let mut char_vec: Vec<char> = int_to_str.chars().collect();
    char_vec.sort(); // ascending vector
    let mut reversed_vec = char_vec.to_vec();
    reversed_vec.reverse(); // reverse is now the higher number

    // turn vectors back into strings
    let smaller_str = char_vec.iter().cloned().collect::<String>();
    let rev_str = reversed_vec.iter().cloned().collect::<String>();

    // strings to integers
    let smaller_int: i16 = smaller_str.parse().unwrap();
    let larger_int: i16 = rev_str.parse().unwrap();

    // find difference between the two
    let result = larger_int - smaller_int;

    if result == 6174 {
        return loop_counter + 1;
    } else {
        return parse_and_iterate(loop_counter + 1, result);
    }
}

pub fn kaprekar(num: i16) -> i16 {
    if num > 9999 || num < 1000 {
        return -1;
    } else if num == 6174 {
        return 0;
    }

    return parse_and_iterate(0, num);
}
