#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    let mut height_to_num = std::collections::HashMap::new();
    for (i, a) in (&aa).iter().enumerate() {
        height_to_num.insert(*a, i + 1);
    }
    aa.sort_unstable();
    aa.reverse();
    let s = aa
        .iter()
        .map(|a| height_to_num.get(&a).unwrap().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", s);
}
