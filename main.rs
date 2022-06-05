mod kaprekar;

pub fn main() {
    let answer = kaprekar::kaprekar(7164);
    println!("{}", answer);
}
