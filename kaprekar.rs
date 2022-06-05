fn parse_and_iterate(loop_counter: i16, number: i16) -> i16 {
    let converted_int: String = number.to_string();
    let mut test_vec: Vec<char> = converted_int.chars().collect();
    test_vec.sort();
    let mut reverse = test_vec.to_vec();
    reverse.reverse(); // reverse is now the higher number

    // turn the string vectors back into integers
    // first collect into strings
    let smaller_str = test_vec.iter().cloned().collect::<String>();
    let rev_str = reverse.iter().cloned().collect::<String>();

    // then integers
    let smaller_int: i16 = smaller_str.parse().unwrap();
    let larger_int: i16 = rev_str.parse().unwrap();

    // reverse minus original
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
