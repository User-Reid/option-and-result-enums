use std::num::ParseIntError;

fn main() {
    let text: &str = "30";
    let text_as_number: Result<i8, ParseIntError> = text.parse::<i8>();

    println!("{:?}", text_as_number);

    let text: &str = "Rocket";
    let text_as_number: Result<i8, ParseIntError> = text.parse::<i8>();

    println!("{:?}", text_as_number)
}
