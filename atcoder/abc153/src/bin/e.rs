#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        n: usize,
        mut aabb: [(usize, usize); n]
    };
    aabb.sort_unstable_by_key(|ab| ab.0);
    const N: usize = 10000;
    let mut vv = vec![std::usize::MAX >> 4; N * 2 + 1];
    vv[0] = 0;
    for (a, b) in aabb {
        for i in 0..=N {
            vv[i + a] = vv[i + a].min(vv[i] + b);
        }
    }
    println!("{}", vv[h..].iter().min().unwrap());
}
