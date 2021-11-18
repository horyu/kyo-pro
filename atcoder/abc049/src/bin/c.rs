#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use regex::bytes::Regex;

fn main() {
    input! {
        s: String
    };
    let regex = Regex::new(r#"^(dream|dreamer|erase|eraser)+$"#).unwrap();
    let tf = regex.is_match(s.as_bytes());
    println!("{}", ["NO", "YES"][tf as usize]);
}
