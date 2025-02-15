fn play(instrument: Option<&String>) {
    match instrument {
        Option::Some(instrument) => println!("You are playing the {instrument}"),
        Option::None => print!("You done fucked up cousin"),
    }
}

fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instruments.get(2);
    play(bass);

    let invalid_instrument: Option<&String> = musical_instruments.get(100);
    play(invalid_instrument)
}
