#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        h: usize,
        _w: usize,
        ss: [Chars; h]
    };
    let mut vv = vec![];
    for (i, s) in ss.into_iter().enumerate() {
        for (j, c) in s.into_iter().enumerate() {
            if c == 'o' {
                vv.push((i, j));
            }
        }
    }
    let rs = vv[0].0.abs_diff(vv[1].0) + vv[0].1.abs_diff(vv[1].1);
    println!("{rs}");
}
