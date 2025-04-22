pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

pub fn generate_round_keys(initialKey: &[u8; 16]) -> [u8; 176] {
    let mut round_keys = [0; 176];
    for i in 0..176 {
        round_keys[i] = initialKey[i % 16];
    }
    round_keys
}
