use rand::{rngs::ThreadRng, thread_rng, Rng};
use sha256::digest;

fn main() {
    // Hashing
    let input: String = String::from("hello");
    let value: String = digest(input);
    assert_eq!(
        value,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );

    // Random number
    let mut rng: ThreadRng = thread_rng();
    let random_number: u32 = rng.gen_range(1..=100);
    println!("Random number: {}", random_number);

    println!("Success!");
}
