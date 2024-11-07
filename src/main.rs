use rand::{rngs::ThreadRng, thread_rng, Rng};
use sha256::digest;

fn main() {
    let secret: String = String::from("HelloWorld");

    let mut rng: ThreadRng = thread_rng();
    let random_number: String = rng.gen_range(1..=1000).to_string();

    let hashed_secret: String = hash_secret(secret + &random_number);
    let expected_secret: String = "HelloWorld".to_string();

    if check_secret(hashed_secret, expected_secret) {
        println!("Right!")
    } else {
        println!("Wrong!")
    }
}

fn hash_secret(secret: String) -> String {
    digest(secret)
}

fn check_secret(hashed_secret: String, expected_secret: String) -> bool {
    for i in 1..=1000 {
        let hashed_expected_secret: String = digest(expected_secret.to_owned() + &i.to_string());
        if hashed_expected_secret == hashed_secret {
            return true;
        }
    }
    false
}
