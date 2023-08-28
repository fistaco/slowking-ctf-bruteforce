use std::thread;

mod check;
use check::{check, get_verification_values};

fn main() {
    let hexwords: Vec<String> = construct_hexwords(false);

    let num_threads = 12;
    let mut thread_handles = vec![];
    let hexwords_chunks = hexwords.chunks(hexwords.len() / num_threads);

    for chunk in hexwords_chunks {
        let hexwords_slice: Vec<String> = chunk.iter().cloned().collect();

        let handle = thread::spawn(move || {
            bruteforce_inputs(hexwords_slice);
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}

/// Brute-forces the given 4-character hex word inputs for all possible verification values.
fn bruteforce_inputs(hexwords: Vec<String>) {
    let verification_nums: Vec<u32> = get_verification_values();

    for (i, hexword) in hexwords.iter().enumerate() {
        if i.rem_euclid(hexwords.len() / 10) == 0 {
            println!("Progress: {} / {} hexwords in 1 thread", i, hexwords.len());
        }

        let output: u32 = check(hexword);

        for v in verification_nums.iter() {
            if &output == v {
                println!("Flag part for v = {}: {}", v, hexword);
            }
        }
    }
}

/// Constructs a vector of all 16^4 = 2^16 combinations of 4-character hexadecimal strings.
fn construct_hexwords(for_last_subflag: bool) -> Vec<String> {
    let hex_chars: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let mut hexwords: Vec<String> = Vec::new();

    for c1 in &hex_chars {
        for c2 in &hex_chars {
            for c3 in &hex_chars {
                for c4 in &hex_chars {
                    if for_last_subflag {
                        hexwords.push(format!("{}{}{}}}", c1, c2, c3));
                        break;
                    }
                    let combination = format!("{}{}{}{}", c1, c2, c3, c4);
                    hexwords.push(combination);
                }
            }
        }
    }

    hexwords
}
