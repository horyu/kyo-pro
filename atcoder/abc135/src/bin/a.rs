#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize
    };
    // a - k = b - k => a = b
    // a - k = -b + k => a + b / 2 = k
    let ab = a + b;
    if ab % 2 == 0 {
        println!("{}", ab / 2);
    } else {
        println!("IMPOSSIBLE");
    }
}
