#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.sort();
    aa.dedup();
    loop {
        let min = aa[0];
        let min_kouho = aa
            .iter()
            .map(|&a| a % min + if a % min == 0 { a } else { 0 })
            .min()
            .unwrap();
        if min_kouho < min {
            aa.insert(0, min_kouho);
            continue;
        }
        println!("{}", min);
        return;
    };
}
