mod rsa;
use rsa::{calculate_private_key, decrypt, encrypt};

fn main() {
    println!("Welcome to simple RSA");

    // What number do you want to encrypt?
    // It has to be less than the max
    let message = 6;
    println!("The original message is: {}", message);

    // Select 2 prime numbers
    let prime1 = 7;
    let prime2 = 13;

    println!("The first prime is: {}", prime1);
    println!("The second prime is: {}", prime2);

    // The maximum number before we wrap around
    let max = prime1 * prime2;

    println!("The max is: {}", max);

    // Arbitrary (prime) public and (calculated) private keys
    let public_key = 5;
    let private_key = calculate_private_key(prime1, prime2, public_key);

    println!("The public key is: {}", public_key);
    println!("The private key is: {}", private_key);

    // Encrypt and immediately decrypt the message
    let encrypted_message = encrypt(message, public_key, max);
    let decrypted_message = decrypt(encrypted_message, private_key, max);

    println!("The encrypted message is: {}", encrypted_message);
    println!("The decrypted message is: {}", decrypted_message);
}
