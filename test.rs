use numtoa::NumToA;
pub fn test() {
    let mut buf = [0u8; 20];
    let mut string = String::new();

    for number in (1..10) {
        string.push_str(number.numtoa_str(10, &mut buf));
        string.push('\n');
    }

    println!("{}", string);
}
