// Cargo.toml
// [dependencies]
// rand = "*"
// bincode = { version = "2.0", features = ["serde"] }
// tfhe = { version = "1.2.0", features = ["boolean", "shortint", "integer", "strings"] }

use bincode;
use std::fmt;
use std::time::Instant;
use tfhe::core_crypto::prelude::*;
use tfhe::prelude::*;
use tfhe::FheUint8;
use tfhe::{generate_keys, set_server_key, ClientKey, ConfigBuilder};

const UP_LOW_DISTANCE: u8 = 32;

struct FheAsciiString {
    bytes: Vec<FheUint8>,
}

impl fmt::Display for FheAsciiString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let byte_array = bincode::serde::encode_to_vec(&self.bytes, bincode::config::standard())
            .expect("Failed to serialize FheAsciiString");
        write!(f, "{}", vec_to_hex(byte_array))
    }
}

fn vec_to_hex(bytes: Vec<u8>) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn to_upper(c: &FheUint8) -> FheUint8 {
    c - FheUint8::cast_from(c.gt(96) & c.lt(123)) * UP_LOW_DISTANCE
}

fn to_lower(c: &FheUint8) -> FheUint8 {
    c + FheUint8::cast_from(c.gt(64) & c.lt(91)) * UP_LOW_DISTANCE
}

impl FheAsciiString {
    fn encrypt(string: &str, client_key: &ClientKey) -> Self {
        let fhe_bytes: Vec<FheUint8> = string
            .bytes()
            .map(|b| FheUint8::encrypt(b, client_key))
            .collect();
        Self { bytes: fhe_bytes }
    }

    fn decrypt(&self, client_key: &ClientKey) -> String {
        let ascii_bytes: Vec<u8> = self
            .bytes
            .iter()
            .map(|fhe_b| fhe_b.decrypt(client_key))
            .collect();
        String::from_utf8(ascii_bytes).unwrap()
    }

    fn to_upper(&self) -> Self {
        Self { bytes: self.bytes.iter().map(to_upper).collect() }
    }

    fn to_lower(&self) -> Self {
        Self { bytes: self.bytes.iter().map(to_lower).collect() }
    }
}

// A function to generate an alphabetic string
fn generate_string(length: usize) -> String {
    use rand::distr::Alphabetic;
    use rand::{rng, Rng};
    let res = rng().sample_iter(&Alphabetic).take(length).collect();
    String::from_utf8(res).unwrap()
}

fn main() {
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);

    set_server_key(server_key);

    let test_string = generate_string(32);
    println!("Test string: {test_string} length {}", test_string.len());

    let mut before = Instant::now();
    let encrypted_object = FheAsciiString::encrypt(&test_string, &client_key);
    let encrypted_string = format!("{encrypted_object}"); // Hexadecimal
    let encrypt_time = before.elapsed();
    println!(
        "Encrypted string: {encrypted_string:.64}... size {:.2?} time {:.2?}",
        encrypted_string.len() / 2,
        encrypt_time
    );

    before = Instant::now();
    let decrypted_string = encrypted_object.decrypt(&client_key);
    let decrypt_time = before.elapsed();
    println!(
        "Decrypted string: {decrypted_string} time {:.2?}",
        decrypt_time
    );
    assert_eq!(decrypted_string, test_string);

    before = Instant::now();
    let encrypted_object_upper = encrypted_object.to_upper();
    let upper_time = before.elapsed();
    let decrypted_string = encrypted_object_upper.decrypt(&client_key);
    println!("Upper string: {decrypted_string} time {:.2?}", upper_time);
    assert_eq!(decrypted_string, test_string.to_uppercase());

    before = Instant::now();
    let encrypted_object_lower = encrypted_object_upper.to_lower();
    let lower_time = before.elapsed();
    let decrypted_string = encrypted_object_lower.decrypt(&client_key);
    println!("Lower string: {decrypted_string} time {:.2?}", lower_time);
    assert_eq!(decrypted_string, test_string.to_lowercase());
}
