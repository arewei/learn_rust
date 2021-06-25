use rand::Rng;

#[allow(dead_code)]
pub fn main1() {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    println!("{}", n1);
    println!("{}", rng.gen::<f64>());
}