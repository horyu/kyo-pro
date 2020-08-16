#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String
    };
    let slen = s.len();
    for i in 1..slen {
        let len = slen - i;
        if len % 2 == 0 {
            let half = len / 2;
            if s[0..half] == s[half..len] {
                println!("{}", len);
                return;
            }
        }
    }
}
