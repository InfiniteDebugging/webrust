use std::io;
use weblib::cipher::{rot13, rsa, Cipher};

fn main() {
    println!("Hello, world!");
    println!("Let's encrypt something. Write something:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read input!");
    let user_input = user_input.trim_end_matches('\n');
    println!("Your input: {}", &user_input);
    println!(
        "Your input Rot13 encrypted: {}",
        rot13::Rot13(user_input.to_string())
            .encrypted_string()
            .unwrap()
    );
    let rsa_encrypted = rsa::Rsa::new(user_input.to_string()).expect("Error while encrypting");
    println!(
        "Your input RSA encrypted: {}",
        rsa_encrypted.encrypted_string().unwrap()
    );
    println!(
        "Your input RSA encrypted and decrypted: {}",
        rsa_encrypted.original_string().unwrap()
    );
}
