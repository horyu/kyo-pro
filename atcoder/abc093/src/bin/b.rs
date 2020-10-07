#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize,
        k: isize
    };
    // a ..= a + k - 1
    //       b - k + 1 ..= b
    let amax = a + k - 1;
    let bmin = b - k + 1;
    if amax >= bmin {
        for i in a..=b {
            println!("{}", i);
        }
    } else {
        for i in (a..=amax).chain(bmin..=b) {
            println!("{}", i);
        }
    }
}
