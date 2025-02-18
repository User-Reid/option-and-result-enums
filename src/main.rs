fn operation(great_success: bool) -> Result<&'static str, &'static str> {
    if great_success {
        Ok("Success")
    } else {
        Err("Failure")
    }
}

fn main() {
    let my_result: Result<&str, &str> = operation(true);

    let content = match my_result {
        Ok(message) => message,
        Err(error) => error,
    };

    println!("{}", my_result.unwrap());
}
