#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut n: usize
    };
    let mut s = Vec::new();
    while n > 0 {
        n -= 1;
        s.push((b'a' + ((n % 26) as u8)) as char);
        n /= 26;
    }
    println!("{}", s.iter().rev().collect::<String>());
}
