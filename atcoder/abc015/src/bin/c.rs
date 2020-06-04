#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ttt: [[usize; k]; n]
    };
    for indexes in (0..k).combinations_with_replacement(n) {
        for indexes in indexes.into_iter().permutations(n) {
            // println!("{:?}", indexes);
            let mut num = 0;
            for (tt_index, t_index) in indexes.into_iter().enumerate() {
                num ^= ttt[tt_index][t_index];
            }
            if num == 0 {
                println!("Found");
                return;
            }
        }
    }
    println!("Nothing");
}
