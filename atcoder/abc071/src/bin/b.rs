#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let mut arr = [true; 128];
    for c in s {
        arr[c as usize] = false;
    }
    for i in b'a'..=b'z' {
        if arr[i as usize] {
            println!("{}", i as char);
            return;
        }
    }
    println!("None");
}
