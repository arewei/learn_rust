use std::collections::HashMap;

pub fn test_hashmap() {
    let mut come_from = HashMap::new();

    come_from.insert("WaySLOG", "HeBei");
    come_from.insert("Marisa", "U.S.");
    come_from.insert("Mike", "HuoGuo");

    if !come_from.contains_key("elthon") {
        println!("Oh, 我们查到了{}个人，但是可怜的Elton猫还是无家可归", come_from.len());
    }

    come_from.remove("Mike");
    println!("Mike猫的家乡不是火锅！不是火锅！不是火锅！虽然好吃！");

    let who = vec!["MoGu", "Marisa"];
    for person in &who {
        match come_from.get(person) {
            Some(location) => println!("{} 来自 {}", person, location),
            None => println!("{} 也无家可归", person),
        }
    }

    println!("所有人？");
    for (person, location) in &come_from {
        println!("{} 来自 {}", person, location);
    }

    for person in &who {
        println!("{:?}", come_from.entry(person))
    }
}