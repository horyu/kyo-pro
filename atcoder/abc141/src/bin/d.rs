#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n]
    };
    let mut bh: BinaryHeap<_> = aa.into_iter().collect();
    for _ in 0..m {
        if let Some(a) = bh.pop() {
            bh.push(a / 2);
        }
    }
    let rs = bh.into_iter().sum::<usize>();
    println!("{}", rs);
}
