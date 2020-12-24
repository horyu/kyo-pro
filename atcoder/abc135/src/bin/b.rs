#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n]
    };
    let invalids = (0usize..n).filter(|&i| i != pp[i]).count();
    println!("{}", ["NO", "YES"][[0, 2].contains(&invalids) as usize]);
}
