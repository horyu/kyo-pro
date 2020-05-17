#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut n: usize
    };
    n %= 30;
    let mut arr = [1, 2, 3, 4, 5, 6];
    for i in 0..n {
        arr.swap(i % 5, i % 5 + 1);
    }
    for x in &arr {
        print!("{}", x)
    }
    print!("\n");
}
