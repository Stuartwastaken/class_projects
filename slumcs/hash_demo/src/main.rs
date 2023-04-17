use std::env;
use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;

fn print_hex(array: &[u8]) {
    for byte in array {
        print!("{:0>2x}", byte);
    }
    print!("\n");
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let iter = args[1].parse::<u32>().unwrap();
    let keylen = args[2].parse::<usize>().unwrap();
    let salt = args[3].as_bytes();
    let mut dk = vec![0u8; keylen];
    let hash = args[4].as_bytes();

    pbkdf2::<Hmac<Sha256>>(hash, salt, iter, &mut dk);

    let hash2 = hex::encode(dk);

    println!("hash2: {}", hash2);
    // let args: Vec<String> = env::args().collect();

    // if args.len() != 4 {
    //     println!("Usage: cargo run -- iterations salt password");
    //     return;
    // }

    // let iterations: u32 = match args[1].parse() {
    //     Ok(x) => x,
    //     _ => {
    //         println!("Enter valid u32 for argument 1");
    //         return;
    //     }
    // };
    // let salt = &args[2];
    // let password = &args[3];

    // // Confirm the program input
    // println!(
    //     "Got password: '{password}', salt: '{salt}', and iterations: {iterations}"
    // );

    // // This function takes a reference to an array, and stores the result hash
    // // in that array. May be faster for doing many repeated hashes.
    // let mut hash1 = [0u8; 20];
    // pbkdf2_hmac::<Sha256>(password.as_bytes(), salt.as_bytes(), iterations, &mut hash1);

    // // Print output first with the debug formatter, then with our custom function
    // println!("hash1: {:x?}", hash1);
    // print_hex(&hash1);

    // // This function returns the output as an array
    // let hash2 = pbkdf2_hmac_array::<Sha256, 20>(password.as_bytes(), salt.as_bytes(), iterations);
    // println!("hash2: {:x?}", hash2);
    // print_hex(&hash2);
}
