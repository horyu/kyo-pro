#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
        ww: [usize; n],
    };
    let mut cc = vec![];
    let mut aa = vec![];
    for (i, &w) in ww.iter().enumerate() {
        if s[i] == '0' {
            cc.push(w);
        } else {
            aa.push(w);
        }
    }
    cc.sort_unstable();
    aa.sort_unstable();

    let mut rs = 0;
    for w in ww {
        let tmp = cc.partition_point(|&c| c < w) + aa.len() - aa.partition_point(|&c| c < w);
        rs = rs.max(tmp);
        let w = w + 1;
        let tmp = cc.partition_point(|&c| c < w) + aa.len() - aa.partition_point(|&c| c < w);
        rs = rs.max(tmp);
    }
    println!("{rs}");
}
