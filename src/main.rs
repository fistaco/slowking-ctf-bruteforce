#![allow(unused)]
mod check;
use check::{check,get_verification_values};

fn main() {
    let hexwords: Vec<String> = construct_hexwords();
    let verification_nums: Vec<u32> = get_verification_values();
 
    for v in &verification_nums[1..8] { // Words 0 and 8 contain non-hex characters
        for hexword in hexwords.iter() {
            println!("Checking 32-bit hexadecimal word {}...", hexword);
            let mut input_is_flag_part: bool = check(hexword, v);
        }
    }
}

fn construct_hexwords() -> Vec<String> {
    let hex_chars: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let mut hexwords: Vec<String> = Vec::new();

    for c1 in &hex_chars {
        for c2 in &hex_chars {
            for c3 in &hex_chars {
                for c4 in &hex_chars {
                    let combination = format!("{}{}{}{}", c1, c2, c3, c4);
                    hexwords.push(combination);
                }
            }
        }
    }

    hexwords
}
