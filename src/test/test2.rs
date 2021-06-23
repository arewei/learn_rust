pub fn test2() {
    let day = 5;

    match day {
        1 | 6 => println!("weekend"),
        1 ..= 5 => println!("weekday"),
        _ => println!("invalid"),
    }
}