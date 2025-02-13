fn main() {
    let a: Option<i32> = Option::Some(5);
    let b: Option<&str> = Option::Some("Taco");
    let c: Option<bool> = Option::Some(true);

    let a = Option::<i8>::Some(5);

    let d: Option<&str> = Option::None;
}
