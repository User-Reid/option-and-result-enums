fn main() {
    let ok: Result<i32, &str> = Ok(5);
    let disaster: Result<i8, &str> = Err("Something bad happened");

    println!("{ok:?}");
    println!("{disaster:?}")
}
