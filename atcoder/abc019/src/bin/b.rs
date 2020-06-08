#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let mut pre_char = s[0];
    let mut count = 1;
    for &c in &s[1..] {
        if c != pre_char {
            print!("{}{}", pre_char, count);
            pre_char = c;
            count = 1;
        } else {
            count += 1;
        }
    }
    println!("{}{}", pre_char, count);
}
