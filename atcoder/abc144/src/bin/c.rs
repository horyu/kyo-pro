#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let mut min = n - 1;
    for i in (2usize..).take_while(|x| x * x <= n) {
        if n % i == 0 {
            min = min.min((n / i - 1) + (i - 1));
        }
    }
    println!("{}", min);
}
