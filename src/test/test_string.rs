#[allow(dead_code)]
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

    println!("{}", s);


    let mut x = 5;

    match x {
        ref mut mr => println!("mut ref: {}", mr),
    }

    let ref mut _mrx = x;

    // std::f64::consts::PI

    let mut x: Vec<i32> = vec!(1i32, 2, 3);

    x.push(100);

    let v = vec![1, 2, 3];
    assert_eq!(v.get(1), Some(&2));
    assert_eq!(v.get(3), None);
}

#[allow(dead_code)]
fn foo<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if true {
        x
    } else {
        y
    }
}

#[allow(dead_code)]
struct Person<'a> {
    age: &'a u8,
}

#[allow(dead_code)]
impl<'a, 'b> Person<'a> {
    fn get_max_age(&'a self, p: &'a Person) -> &'a u8 {
        if self.age > p.age {
            self.age
        } else {
            p.age
        }
    }
}

#[allow(dead_code)]
impl<'a> Person<'a> {
    fn get_age(&self) -> &u8 {
        self.age
    }
}