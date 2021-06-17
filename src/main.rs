use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}
