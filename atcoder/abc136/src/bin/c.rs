#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        hh: [usize; n]
    };
    if n == 1 {
        println!("Yes");
        return;
    }
    let mut pre = hh[0] - 1;
    for &h in &hh[1..] {
        if pre < h {
            pre = h - 1;
        } else if pre == h {
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
