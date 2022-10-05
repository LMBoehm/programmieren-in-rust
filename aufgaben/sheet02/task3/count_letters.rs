
pub fn count(name : &str, letter: char) -> i32 {
    name.chars().fold(0, |acc, l| if l == letter {acc + 1} else {acc})
}

fn main() {
    println!("{}", count("petper", 'e'));
}
