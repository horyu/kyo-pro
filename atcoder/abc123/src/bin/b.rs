#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut xx: [usize; 5]
    };
    xx.sort_by_key(|x| 10 - if x % 10 == 0 { 10 } else { x % 10 });
    let rs = xx[0..4].iter().fold(xx[4], |acc, &x| {
        acc + if x % 10 == 0 { x } else { (x / 10 + 1) * 10 }
    });
    println!("{}", rs);
}
