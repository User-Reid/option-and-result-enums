#[derive(Debug, Clone, Copy)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => {
                println!("{value}");
                return value;
            }
            MyOption::None => panic!("YOU DONE FUCKED UP COUSIN"),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

fn main() {
    let some_option: MyOption = MyOption::Some(12);
    let none_option: MyOption = MyOption::None;

    some_option.unwrap();
    // none_option.unwrap();
    println!("{}", none_option.unwrap_or(20));
}
