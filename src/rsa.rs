pub fn decrypt(encrypted_message: i32, private_key: i32, max: i32) -> i32 {
    let mut decrypted_message = 1;

    for _ in 0..private_key {
        decrypted_message = (decrypted_message * encrypted_message) % max;
    }

    decrypted_message
}

pub fn encrypt(message: i32, public_key: i32, max: i32) -> i32 {
    // multiply message by itself public_key times then mod by max
    let mut encrypted_message = 1;

    for _ in 0..public_key {
        encrypted_message = (encrypted_message * message) % max;
    }

    encrypted_message
}

fn extended_euclidean_algorithm(a: i32, b: i32) -> i32 {
    let mut x0 = 1;
    let mut x1 = 0;
    let mut y0 = 0;
    let mut y1 = 1;
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let quotient = a / b;
        let remainder = a % b;
        a = b;
        b = remainder;

        let next_x = x0 - quotient * x1;
        let next_y = y0 - quotient * y1;

        x0 = x1;
        x1 = next_x;
        y0 = y1;
        y1 = next_y;
    }

    if x0 < 0 {
        x0 += b;
    }

    x0
}

pub fn calculate_private_key(p: i32, q: i32, public_key: i32) -> i32 {
    let phi = (p - 1) * (q - 1);
    let private_key = extended_euclidean_algorithm(public_key, phi);

    private_key
}
