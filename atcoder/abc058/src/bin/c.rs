#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n]
    };
    let mut vv = vec![vec![0; n]; 26];
    for (index, s) in ss.iter().enumerate() {
        for &c in s {
            vv[((c as u8) - b'a') as usize][index] += 1;
        }
    }
    let mut s = String::new();
    for (index, v) in vv.iter().enumerate() {
        for _ in 0..*v.iter().min().unwrap() {
            s.push((b'a' + index as u8) as char);
        }
    }
    println!("{}", s);
}
