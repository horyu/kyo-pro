#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut x: u128,
        y: u128,
        a: u128,
        b: u128
    };
    // *aが+b以下であるうちは*a、以降は限界まで+b
    let mut exp = 0u128;
    while x * a <= x + b {
        x *= a;
        if x >= y {
            println!("{}", exp);
            return;
        }
        exp += 1;
    }
    exp += (y - x - 1) / b;
    println!("{}", exp);
}
