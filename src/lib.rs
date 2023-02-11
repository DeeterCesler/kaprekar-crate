pub fn calculate_three(num: i32) -> Option<i32> {
    if !(100..=999).contains(&num) {
        None
    } else if num == 495 {
        Some(0)
    } else {
        parse_and_iterate(0, num, 3)
    }
}

pub fn calculate_four(num: i32) -> Option<i32> {
    if !(1000..=9999).contains(&num) {
        None
    } else if num == 6174 {
        Some(0)
    } else {
        parse_and_iterate(0, num, 4)
    }
}

pub fn parse_and_iterate(loop_counter: i32, number: i32, num_of_digits: i8) -> Option<i32> {
    let int_to_str: String = number.to_string();
    let mut char_vec: Vec<char> = int_to_str.chars().collect();
    char_vec.sort_unstable(); // ascending vector
    let mut reversed_vec = char_vec.to_vec();
    reversed_vec.reverse(); // reverse is now the higher number

    // turn vectors back into strings
    let smaller_str = char_vec.iter().cloned().collect::<String>();
    let rev_str = reversed_vec.iter().cloned().collect::<String>();

    // strings to integers
    let smaller_int: i32 = smaller_str.parse().unwrap();
    let larger_int: i32 = rev_str.parse().unwrap();

    // find difference between the two
    let mut result = larger_int - smaller_int;

    // make sure, if there are zeros in the number,
    // they still get added to the end of the large number
    // e.g. original: 1040 => small: 0014 => large: 4100
    if num_of_digits == 3 {
        if result < 10 {
            result *= 100
        } else if result < 100 {
            result *= 10
        }

        if result == 495 {
            Some(loop_counter + 1)
        } else if result == 0 {
            None
        } else {
            parse_and_iterate(loop_counter + 1, result, 3)
        }
    } else {
        if result < 10 {
            result *= 1000
        } else if result < 100 {
            result *= 100
        } else if result < 1000 {
            result *= 10
        }

        if result == 6174 {
            Some(loop_counter + 1)
        } else if result == 0 {
            None
        } else {
            parse_and_iterate(loop_counter + 1, result, 4)
        }
    }
}

pub fn main() {
    let answer_four = calculate_four(8811);
    let answer_three = calculate_three(888);
    println!("four-digit: {:#?}", answer_four);
    println!("three-digit: {:#?}", answer_three);
}
