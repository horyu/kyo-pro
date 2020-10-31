#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
        k: u128
    };
    let s: Vec<_> = s.iter().map(|&c| c.to_digit(10).unwrap() as u128).collect();
    let mut sum = 0u128;
    for c in s {
        if c == 1 {
            sum += 1;
            if sum >= k {
                println!("{}", c);
                return;
            }
        } else {
            let mut csize = 1;
            for _ in 0..k {
                csize *= c;
                if sum + csize >= k {
                    println!("{}", c);
                    return;
                }
            }
            sum += csize;
        }
    }
}
