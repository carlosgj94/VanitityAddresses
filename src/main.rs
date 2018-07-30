extern crate arrayvec;
extern crate base58;
extern crate crypto;
extern crate rand;
extern crate rustc_serialize;
extern crate secp256k1;

use base58::ToBase58;
use crypto::digest::Digest;
use crypto::ripemd160::Ripemd160;
use crypto::sha2::Sha256;
use rand::os::OsRng;
pub use rustc_serialize::hex::ToHex;
use secp256k1::Error;
use std::io::{stdin, stdout, Write};
use std::thread;

fn main() {
    let chars = get_string();
    let mut found = false;
    let mut public: secp256k1::key::PublicKey = secp256k1::key::PublicKey::new();
    let mut index = 0;
    while !found {
        let chars = chars.clone();
        let mut address: String = String::from("Init");
        let handle = thread::spawn(move || {
            public = get_public_key();
            address = get_bitcoin_address(public);
            let slice = &address[1..chars.len() + 1];
            println!("{}: {}", index, address);
            if chars == slice {
                found = true;
            }
            //thread::sleep(Duration::from_secs(1));
        });
        index += 1;
        if index % 6 == 0 {
            handle.join().unwrap();
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

fn get_public_key() -> secp256k1::key::PublicKey {
    let context = secp256k1::Secp256k1::new();
    let rng = OsRng::new().map_err(|_| Error::InvalidSecretKey).ok();
    let res = context.generate_keypair(&mut rng.unwrap());
    let (_secret, public) = res.unwrap();
    // public.serialize_vec(&context, false).as_slice().to_base58()
    public
}

fn get_bitcoin_address(public: secp256k1::key::PublicKey) -> String {
    let context = secp256k1::Secp256k1::new();
    let mut sha256 = Sha256::new();
    let mut ripemd = Ripemd160::new();
    sha256.input(&public.serialize_vec(&context, false).as_slice());
    ripemd.input_str(&sha256.result_str());
    let mut result = [0u8; 25];
    let mut checksum = [0u8; 32];
    ripemd.result(&mut result[1..21]);
    sha256.reset();
    sha256.input(&result[..21]);
    sha256.result(&mut checksum);
    sha256.reset();
    sha256.input(&checksum);
    sha256.result(&mut checksum);
    result[21..25].copy_from_slice(&*checksum[0..4].to_owned());
    result.to_base58()
}
