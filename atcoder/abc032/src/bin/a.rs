#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: i32,
        b: i32,
        n: i32
    };
    for i in 0..std::i32::MAX {
        let num = n + i;
        if (num % a == 0) && (num % b == 0) {
            println!("{}", num);
            return;
        }
    }
}
