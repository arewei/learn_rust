pub fn test_string() {
    let x = "哎呦我去".to_string();
    for i in x.as_bytes() {
        print!("{} ", i);
    }

    println!();

    for i in x.chars() {
        print!("{} ", i)
    }

    println!();

    println!("nth 2: {:?}", x.chars().nth(2));

    let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子", 81, "wayslog", width = 4, height = 178);

    println!("{}", s)
}