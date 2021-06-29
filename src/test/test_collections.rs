use std::collections::HashMap;

#[allow(dead_code)]
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

    let m = (1..20).fold(1usize, |mul, x| mul * x);
    println!("m: {}", m);

    let v = vec![1, 2, 3, 4, 5, 6];
    let v_take = v.iter()
        .cloned()
        .take(2)
        .collect::<Vec<_>>();
    assert_eq!(v_take, vec![1, 2]);

    let names = vec!["WaySLOG", "Mike", "Elton"];
    let scores = vec![60, 80, 100];
    let score_map: HashMap<_, _> = names.iter()
        .zip(scores.iter())
        .collect();
    println!("{:?}", score_map);

    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter()
        .enumerate()
        // 迭代生成标，并且每两个元素剔除一个
        .filter(|&(idx, _)| idx % 2 == 0)
        // 将下标去除,如果调用unzip获得最后结果的话，可以调用下面这句，终止链式调用
        // .unzip::<_,_, vec<_>, vec<_>>().1
        .map(|(_, val)| val)
        // 累加 1+3+5 = 9
        .fold(0u64, |sum, acm| sum + acm);


    println!("{}", val);
}