mod kaprekar;

pub fn main() {
    let answer = kaprekar::kaprekar(3321);
    println!("{}", answer);
}
