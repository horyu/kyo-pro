#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    for i in 0..=(n / 7) {
        if (n - 7 * i) % 4 == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
