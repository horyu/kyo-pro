#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        w: Chars
    };
    let mut arr = vec![0; 1usize + b'z' as usize];
    for c in w {
        arr[c as usize] += 1;
    }
    for c in b'a'..=b'z' {
        if arr[c as usize] % 2 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
