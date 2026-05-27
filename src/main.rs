//Rusty XOR Cipher
use std::io;

fn xor_cipher(read: &[u8], write: &mut Vec<u8>, key:u8) {
    for c in read{
        write.push( (*c as u8) ^ key);
    }
}

fn main() {
    let mut plaintext = String::new();
    println!("Enter the text you wanna encrypt/decrypt: ");
    io::stdin().read_line(&mut plaintext).expect("There was an error while reading text");


    let mut key_str = String::new();
    println!("Enter key: ");
    io::stdin().read_line(&mut key_str).expect("There was an error while reading text");

    let key: u8 = key_str
        .trim()
        .parse::<u8>()
        .expect("Please enter a valid number between 0 and 255");


    let plain_bytes = plaintext.trim().to_string().into_bytes();    

    let mut encrypted_bytes = Vec::new();
    xor_cipher(&plain_bytes, &mut encrypted_bytes, key);

    let mut encryped_bytes = Vec::new();
    xor_cipher(&encrypted_bytes, &mut encryped_bytes, key);    

    println!("Text: {}", plaintext.trim());
    println!("Text in bytes: {:?}", plain_bytes);
    println!("Encrypted bytes: {:?}", encrypted_bytes);
    println!("Decrypted Text: {}",String::from_utf8(encryped_bytes).expect("invalid utf-8"))
    
}
