use std::io::stdin;

extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

extern crate chrono;
use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};

fn process(year: &str, cmp: &str) -> String {
    for month in 1..13 {
        for day in 1..32 {
            let date = format!("{}-{:0>2}-{:0>2}T00:00:00+04:00", year, month, day);
            let time_mark = DateTime::parse_from_rfc3339(&date);
            let time_unwrapped = match time_mark {
                Ok(res) => res.into(),
                Err(_) => SystemTime::now()
            };
            let duration = time_unwrapped.duration_since(UNIX_EPOCH).unwrap().as_secs();

            let mut bytes: [u8; 32] = [0; 32];
            let copy = duration.to_be_bytes();
            for i in 0..copy.len() {
                bytes[32 - copy.len() + i] = copy[i];
            }
            let mut hasher = Sha3::keccak256();
            hasher.input(&bytes);
            let result = hasher.result_str();
            if result == cmp {
                let string = format!("{:0>2}.{:0>2}", day, month);
                return string;
            }
        }
    }
    let result = String::new();
    return result;
}

fn main() {
    let mut year = String::new();
    stdin().read_line(&mut year).unwrap(); year.pop();
    println!("{}", year);
    let mut hash = String::new();
    stdin().read_line(&mut hash).unwrap(); hash.pop();
    println!("{}", hash);

    println!("{}", process(&year, &hash));
}