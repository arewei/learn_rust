fn main() {
    let s1 = String::from("abc");

    {
        let s2 = &s1;
        println!("s2 = {}", s2);
    }


    
    println!("s1 = {}", s1);

    println!("Hello, world!");
}
