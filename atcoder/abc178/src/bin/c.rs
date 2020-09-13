#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    if n == 1 {
        println!("0");
        return;
    }
    // 10^n - 2*9^n + 8^n
    let (mut a, mut b, mut c) = (10isize, 9isize, 8isize);
    for _ in 1..n {
        a = a * 10 % 1000000007;
        b = b * 9 % 1000000007;
        c = c * 8 % 1000000007;
    }
    let mut rs = (a - 2 * b + c) % 1000000007;
    if rs < 0 {
        rs += 1000000007;
    }
    println!("{}", rs);
}
