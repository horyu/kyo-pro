#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        t: isize,
        aa: [isize; n]
    };
    let mut sum = 0;
    let mut pre = std::isize::MIN;
    for a in aa {
        sum += t;
        if a < pre + t {
            sum -= pre + t - a;
        }
        pre = a;
    }
    println!("{}", sum);
}
