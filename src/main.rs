fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Some(true)
    } else if item_is_in_system {
        Some(false)
    } else {
        None
    }
}

fn main() {
    let availability: Option<bool> = is_item_in_stock(false, true);

    match availability {
        Some(true) => println!("Item is available"),
        Some(false) => println!("Yo shit aint here cousin"),
        None => println!("What the hell are you talking about cuz"),
    }
}
