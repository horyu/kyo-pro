#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize
    };
    if x <= 3 {
        println!("1");
        return;
    }
    let mut max = 1;
    for i in 2..x {
        let mut num = i;
        while num <= x {
            num *= i;
        }
        num /= i;
        if (num != i) && (num > max) {
            max = num;
        }
    }
    println!("{}", max);
}
