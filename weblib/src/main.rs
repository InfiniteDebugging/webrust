use std::io;
use weblib::cipher::{rot13, Cipher};

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
        "Your input encrypted: {}",
        rot13::Rot13(user_input.to_string()).encrypted_string()
    );
}
