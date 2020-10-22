#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let mut tmp = n;
    let mut sn = 0;
    while tmp > 0 {
        sn += tmp % 10;
        tmp /= 10;
    }
    println!("{}", ["No", "Yes"][(n % sn == 0) as usize]);
}
