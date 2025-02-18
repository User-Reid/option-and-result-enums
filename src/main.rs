fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("You fucked up cousin"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let result: Result<f64, String> = divide(2.0, 1.0);

    match result {
        Ok(value) => println!("The result is {value}"),
        Err(message) => println!("Error: {message}"),
    }
}
