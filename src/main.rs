mod rsa;

fn main() {
    // What number do you want to encrypt?
    // It has to be less than the max
    let message = 4;

    let prime1 = 2;
    let prime2 = 3;

    let max = prime1 * prime2;

    // Arbitrary (prime) public and (pre-calculated) private keys
    let public_key = 5;
    let private_key = rsa::calculate_private_key(prime1, prime2, public_key);

    // Encrypt and immediately decrypt the message
    let encrypted_message = rsa::encrypt(message, public_key, max);
    let decrypted_message = rsa::decrypt(encrypted_message, private_key, max);

    println!("Welcome to simple RSA");
    println!("The original message is: {}", message);
    println!("The max is: {}", max);
    println!("The public key is: {}", public_key);
    println!("The private key is: {}", private_key);
    println!("The encrypted message is: {}", encrypted_message);
    println!("The decrypted message is: {}", decrypted_message);
}
