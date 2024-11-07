use sha256::digest;

fn main() {
    let secret: String = "Dipesh78".to_string();
    let hashed_secret: String = hash_secret(secret);
    let expected_secret: String = "Dipesh78".to_string();
    check_secret(hashed_secret, expected_secret);
}

fn hash_secret(secret: String) -> String {
    digest(secret)
}

fn check_secret(hashed_secret: String, expected_secret: String) {
    let hashed_expected_secret: String = digest(expected_secret);
    if hashed_expected_secret == hashed_secret {
        println!("Yes")
    } else {
        println!("No")
    }
}
