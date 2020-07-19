#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let mut pre = s[0];
    let mut count = 0;
    for c in s {
        if c != pre {
            count += 1;
            pre = c;
        }
    }
    print!("{}", count);
}
