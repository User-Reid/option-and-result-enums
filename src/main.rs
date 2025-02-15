fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}

fn main() {
    let availability: Option<bool> = is_item_in_stock(false, true);

    match availability {
        Option::Some(true) => println!("Item is available"),
        Option::Some(false) => println!("Yo shit aint here cousin"),
        Option::None => println!("What the hell are you talking about cuz"),
    }
}
