#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    if n == 2 {
        println!("1");
        return;
    }
    // c>0 の範囲でaとbは自由なので, n-1以下になる整数の組み合わせ総数が解
    let mut count = 0;
    for a in 1..=(n - 1) {
        count += (n - 1) / a;
    }
    println!("{}", count);
}
