use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;
use hex_literal::hex;

//Remember to run only one test at a time:
//Run with: cargo test -- --nocapture --test-threads=1
//
//Can also run individually:
//Run with: cargo run -- iters salt target_hash


fn print_hex(array: &[u8]) {
    for byte in array {
        print!("{:0>2x}", byte);
    }
    print!("\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!("Usage: cargo run -- keysize iterations salt target threads");
        return;
    }

    let keysize: usize = match args[1].parse(){
        Ok(x) => x,
        _ => {println!("Must enter valid integer for first argument"); return}
    };
    let iterations: u32 = match args[2].parse(){
        Ok(x) => x,
        _ => {println!("Must enter valid integer for second argument"); return}
    };
    let salt = &args[3];
    let target = &args[4];
    let threads: u8 = match args[5].parse(){
        Ok(x) => x,
        _ => {println!("Must enter valid integer for fifth argument"); return}
    };

    println!("Cracking with keysize: {}, iterations: {}, salt: {}, target: {}, and threads: {}", keysize, iterations, salt, target, threads);

    let result = crack(keysize, iterations, &salt.as_bytes(), &hex::decode(target).unwrap(), threads);
    if let Some(password) = result {
        println!("Found password: {}", password);
    } else {
        println!("Password not found");
    }
    crack( keysize, iterations, &salt.as_bytes(), &target.as_bytes(), threads );
}

//Tests all passwords from 'a' to 'aaaa' (length dependent on keysize) and
//checks whether they hash to the target hash. If a password is found, returns
//Some(password), otherwise returns None.

pub fn crack(max_keysize: usize, iterations: u32, salt: &[u8], target: &[u8], threads: u8) -> Option<String> {
    let charset: Vec<char> = ('a'..='z').collect();
    let charset_len = charset.len();
    let charset = Arc::new(charset);

    let target = Arc::new(target.to_vec());
    let salt = Arc::new(salt.to_vec());

    let result = Arc::new(Mutex::new(None));

    let mut handles = vec![];

    for i in 0..threads {
        let charset = Arc::clone(&charset);
        let target = Arc::clone(&target);
        let salt = Arc::clone(&salt);
        let result = Arc::clone(&result);

        let handle = thread::spawn(move || {
            for len in 1..=max_keysize {
                let start = (i as usize * charset_len / threads as usize) % charset_len;
                let end = ((i as usize + 1) * charset_len / threads as usize) % charset_len;

                if start < end {
                    if let Some(password) = search_space(start, end, len, &charset, iterations, &salt, &target) {
                        let mut result = result.lock().unwrap();
                        *result = Some(password);
                        break;
                    }
                } else {
                    if let Some(password) = search_space(start, charset_len, len, &charset, iterations, &salt, &target) {
                        let mut result = result.lock().unwrap();
                        *result = Some(password);
                        break;
                    }
                    if let Some(password) =search_space(0, end, len, &charset, iterations, &salt, &target) {
                        let mut result = result.lock().unwrap();
                        *result = Some(password);
                        break;
                        }
                        }
                        }
                        });
                        handles.push(handle);
                    }
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                    
                    let result = result.lock().unwrap();
                    result.clone()
                }

                fn search_space(start: usize, end: usize, len: usize, charset: &[char], iterations: u32, salt: &[u8], target: &[u8]) -> Option<String> {
                let mut buffer = vec!['a'; len];
                for idx in start..end {
                    buffer[0] = charset[idx];
                
                    if let Some(password) = recursive_search(0, len, charset, &buffer, iterations, salt, target) {
                        return Some(password);
                    }
                }
                
                None
            }

            fn recursive_search(depth: usize, max_depth: usize, charset: &[char], buffer: &[char], iterations: u32, salt: &[u8], target: &[u8]) -> Option<String> {
                if depth == max_depth - 1 {
                    let password = buffer.iter().collect::<String>();
                    let mut hash = vec![0u8; target.len()];
                    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, iterations, &mut hash);
            
                    if hash.as_slice() == target {
                        return Some(password);
                    }
                } else {
                    for ch in charset.iter() {
                        let mut new_buffer = buffer.to_vec();
                        new_buffer[depth + 1] = *ch;
            
                        if let Some(password) = recursive_search(depth + 1, max_depth, charset, &new_buffer, iterations, salt, target) {
                            return Some(password);
                        }
                    }
                }
            
                None
            }
            
            






            #[cfg(test)]
            mod password_crack_tests {
                use crate::crack;
                use hex_literal::hex;
                use std::time::{Instant};
            
                //Calls the crack function and doesn't care about the result.
                #[test]
                fn call_crack() {
                    let hash = hex!("8e95be594f2084fcad05981cac19163b54697160");
                    crack( 2, 128, b"na", &hash, 1 );
                }
            
                //Cracks the password "cat" with keysize=3
                #[test]
                fn crack_cat() {
                    let hash = hex!("be3c153739585b98fbb96dd68be71715a311955b");
                    let result = crack( 3, 128, b"na", &hash, 1 );
                    println!("Got test result: {:?}", result );
                    assert!(result.is_some());
                    assert!(result.unwrap() == "cat");
                }
            
                //Cracks the password "cat" with keysize=3 and a different salt from above
                #[test]
                fn crack_cat_diff_salt() {
                    let hash = hex!("ce9b6856926fbc88af08d55d0a12571b18cc35a5");
                    let result = crack( 3, 128, b"xy", &hash, 1 );
                    assert!(result.is_some());
                    assert!(result.unwrap() == "cat");
                }
            
                //Cracks the password "dog" with keysize=4
                #[test]
                fn crack_dog_larger_keysize() {
                    let hash = hex!("6bfe506d99510ddd3ed21c35f9140053e09cbf00");
                    let result = crack( 4, 128, b"na", &hash, 1 );
                    assert!(result.is_some());
                    assert!(result.unwrap() == "dog");
                }
            
                //Cracks the password "pig" using two threads
                #[test]
                fn crack_pig_two_threads() {
                    let hash = hex!("fd1bba12fc118ff663a10796f2b45d5fdde2896b");
                    let result = crack( 3, 128, b"na", &hash, 2 );
                    assert!(result.is_some());
                    assert!(result.unwrap() == "pig");
                }
            
                //Tests whether we test passwords starting with "m" for threads=2
                #[test]
                fn crack_mom_thread_boundary() {
                    let hash = hex!("3228bde24088e047327b5a37f69bf536cc146c71");
                    let result = crack( 3, 128, b"na", &hash, 2 );
                    assert!(result.is_some());
                    assert!(result.unwrap() == "mom");
                }
            
                //Tests whether we test passwords starting with "n" for threads=2
                #[test]
                fn crack_nan_thread_boundary() {
                    let hash = hex!("19b2f089b5c95d777754c2e35e079fdc09c1350f");
                    let result = crack( 3, 128, b"xy", &hash, 2 );
                    assert!(result.is_some());
                    assert!(result.unwrap() == "nan");
                }
            
                //Exhausts the entire search space and times how long it takes to do so
                #[test]
                fn test_speedup() {
                    let hash = b"NoMatchingHashNoMatchingHashNoMatchingHa";
            
                    let start = Instant::now();
            
                    let result = crack( 3, 128, b"na", hash, 1 );
                    assert!(result.is_none());
                    let after_1 = Instant::now();
                    let time_1 = after_1 - start;
                    println!("Time on one thread: {:?}", time_1);
            
                    let result = crack( 3, 128, b"na", hash, 2 );
                    assert!(result.is_none());
                    let after_2 = Instant::now();
                    let time_2 = after_2 - after_1;
                    println!("Time on two threads: {:?}", time_2);
            
                    let result = crack( 3, 128, b"na", hash, 3 );
                    assert!(result.is_none());
                    let after_3 = Instant::now();
                    let time_3 = after_3 - after_2;
                    println!("Time on three threads: {:?}", time_3);
            
                    let result = crack( 3, 128, b"na", hash, 4 );
                    assert!(result.is_none());
                    let after_4 = Instant::now();
                    let time_4 = after_4 - after_3;
                    println!("Time on four threads: {:?}", time_4);
                }
            }