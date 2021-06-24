struct Person {
    name: String,
}

struct RefBoy<'a> {
    loc: &'a i32,
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string()
        }
    }

    fn greeting(&self) {
        println!("{} say hello.", self.name)
    }
}

pub fn test2() {
    let day = 5;

    match day {
        1 | 6 => println!("weekend"),
        1..=5 => println!("weekday"),
        _ => println!("invalid"),
    }
}

pub fn test4() {
    let peter = Person::new("Peter");
    peter.greeting()
}