#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut s: Chars
    };
    let n = s.len();
    if n == 1 {
        println!("0");
        return;
    }
    let mut i = 0usize;
    while i + 1 < s.len() {
        if s[i] != s[i + 1] {
            s.remove(i + 1);
            s.remove(i);
            if i > 0 {
                i -= 1;
            }
            continue;
        }
        i += 1;
    }
    println!("{}", n - s.len());
}
