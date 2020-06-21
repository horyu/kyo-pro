#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [isize; n]
    };
    let sum = aa.iter().sum::<isize>() as usize;
    if sum % n != 0 {
        println!("-1");
        return;
    }
    let num = (sum / n) as isize;
    let mut margin = 0;
    let mut bridge = 0;
    for &a in &aa[..(n - 1)] {
        if (margin == 0) && (a == num) {
            continue;
        }
        margin += a - num;
        if margin != 0 {
            bridge += 1;
        }
    }
    println!("{}", bridge);
}
