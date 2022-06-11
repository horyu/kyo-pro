#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut aa: [usize; n],
        xx: [usize; q],
    };
    aa.sort_unstable();
    let mut ss = vec![0];
    for &a in &aa {
        ss.push(ss.last().unwrap() + a);
    }
    for x in xx {
        let i = aa.partition_point(|&a| a < x);
        // 左側
        let mut rs = x * i - ss[i];
        // 右側
        rs += ss[n] - ss[i] - x * (n - i);
        println!("{rs}");
    }
}
