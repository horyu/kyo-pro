#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        hh: [usize; n]
    };
    if n == 1 {
        println!("1");
        return;
    }
    let mut highest = hh[0];
    let mut count = 1;
    for &h in &hh[1..] {
        if h >= highest {
            highest = h;
            count += 1;
        }
    }
    println!("{}", count);
}
