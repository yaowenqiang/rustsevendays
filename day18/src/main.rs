extern crate crypto;
extern crate rustc_serialize;
extern crate rand;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::aes::{self, KeySize};
use crypto::symmetriccipher::SynchronousStreamCipher;
use rand::{thread_rng, Rng};
use std::iter::repeat;
use rustc_serialize::base64::{STANDARD, ToBase64};

fn main() {
    let input = "hello world";
    let mut sha = Sha256::new();
    sha.input_str(input);
    println!("{}", sha.result_str());
    let mut rng = thread_rng();
    let mut gen[f32; 32]  = rng.gen();
    let mut key : Vec<u8> = repeat(0u8).take(16).collect();
    gen.fill_bytes(&mut key[..]);

    let mut nonce : Vec<u8> = repeat(0u8).take(10).collect();
    gen.fill_bytes(&mut nonce[..]);

    println!("Key: {}", key.to_base64(STANDARD));
    println!("Nonce: {}", nonce.to_base64(STANDARD));

    let mut cipher = aes::ctr(KeySize::KeySize128, &key, &nonce);
    let secret = "I like Nickelback";
    let mut output : Vec<u8> = repeat(078).take(secret.len()).collect();
    cipher.process(secret.as_bytes(), &mut output[..]);
    println!("ciphertext: {}", output.to_base64(STANDARD));

}
