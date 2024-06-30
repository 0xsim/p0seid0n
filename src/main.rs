mod poseidon;
mod utils;

fn main() {
    // Example usage
    let p = BigUint::parse_bytes(b"73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001", 16).unwrap();
    let poseidon = poseidon::PoseidonHash::new(3, 2, 8, 56, p, 32); // 32 is the output length

    let input = "test";
    let hash_hex = poseidon.hash(input);
    println!("Hash: {}", hash_hex);
}
