use core::panic;
use std::str::FromStr;

use num_traits::{ToPrimitive, Zero};


pub fn get_verification_values() -> Vec<u32> {
    vec![
        4125536637,
        631702732,
        2333190013,
        530118337,
        4144883596,
        2748845596,
        1851309964,
        3375630137,
        4104857964
    ]
}

fn fa(x: &i64) -> i64 {
    x*x ^ 3141592653
}

fn fb(x: &i64) -> i64 {
    3*x + 1732050808
}

fn fc(x: &i64) -> i64 {
    x >> 3 | 2236067975
}

fn fd(x: &i64) -> i64 {
    7*x - 2645751308
}

fn bytes_to_int(bytes: &[u8]) -> i64 {
    ((bytes[0] as i64) << 24) +
    ((bytes[1] as i64) << 16) +
    ((bytes[2] as i64) <<  8) +
    ((bytes[3] as i64) <<  0)
}

pub fn check(input: &String, v: &u32) -> bool {
    const MAX_INT: i64 = 4294967296;

    let mut t: i64 = bytes_to_int(input.as_bytes());
    let f: i64 = t.clone();

    for i in 0..13371337 {
        let func_idx: i64 = t.rem_euclid(4);

        t = match func_idx {
            0 => fa(&t),
            1 => fb(&t),
            2 => fc(&t),
            3 => fd(&t),
            _ => panic!("Computed non-existent function index {}.", func_idx)
        } % MAX_INT;
    }

    t = (t + f) % MAX_INT;

    let t_conv_u: u32 = t.to_u32().unwrap();
    t_conv_u == *v
}