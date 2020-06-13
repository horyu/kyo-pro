#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        s: isize,
        t: isize,
        mut w: isize,
        aa: [isize; n - 1]
    };
    let mut count = (s <= w && w <= t) as i32;
    for a in aa {
        w += a;
        if s <= w && w <= t {
            count += 1;
        }
    }
    println!("{}", count);
}
