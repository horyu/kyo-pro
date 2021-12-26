#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        xxyyhh: [(isize, isize, isize); n]
    };
    // h>0 となる要素を先頭に持ってくる
    let xxyyhh = xxyyhh.into_iter().sorted_by_key(|xyh| -xyh.2).collect_vec();
    for cx in 0..=100 {
        for cy in 0..=100 {
            let mut iter = xxyyhh.iter();
            if let Some((x, y, h)) = iter.next() {
                let ch = h + (x - cx).abs() + (y - cy).abs();
                if iter.all(|(x, y, h)| *h == (ch - (x - cx).abs() - (y - cy).abs()).max(0)) {
                    println!("{} {} {}", cx, cy, ch);
                    return;
                }
            }
        }
    }
}
