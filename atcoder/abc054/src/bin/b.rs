#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        aaa: [String; n],
        bbb: [String; m],
    };
    for ai in 0usize..=(n - m) {
        for aj in 0usize..=(n - m) {
            let tf = bbb
                .iter()
                .enumerate()
                .all(|(bi, bb)| bb == &aaa[ai + bi][aj..(aj + m)]);
            if tf {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
