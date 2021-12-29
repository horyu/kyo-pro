#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut ss: [usize; n]
    };
    let sum: usize = ss.iter().sum();
    if sum % 10 != 0 {
        println!("{}", sum);
    } else {
        ss.sort_unstable();
        if let Some(s) = ss.iter().find(|&s| s % 10 != 0) {
            println!("{}", sum - s);
        } else {
            println!("0");
        }
    }
}
