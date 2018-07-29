extern crate base58;
extern crate rand;
extern crate rustc_serialize;
extern crate secp256k1;

use base58::ToBase58;
use rand::os::OsRng;
pub use rustc_serialize::hex::ToHex;
use secp256k1::Error;
use std::io::{stdin, stdout, Write};

fn main() {
    let chars = get_string();
    let mut found = false;
    let mut public: String;
    while !found {
        public = get_private_key();
        let slice = &public[1..chars.len() + 1];
        println!("{}", slice);
        if chars == slice {
            found = true;
        }
    }
}

fn get_string() -> String {
    println!("Hello, you!");
    println!("Let me help you with your address creation");
    println!("Why don't you give a string?");
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn get_private_key() -> String {
    let context = secp256k1::Secp256k1::new();
    let rng = OsRng::new().map_err(|_| Error::InvalidSecretKey).ok();
    let res = context.generate_keypair(&mut rng.unwrap());
    let (_secret, public) = res.unwrap();
    println!(
        "{:?}",
        public.serialize_vec(&context, false).as_slice().to_base58()
    );
    public.serialize_vec(&context, false).as_slice().to_base58()
}
