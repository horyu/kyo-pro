#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    let x = aa
        .iter()
        .tuple_windows()
        .find_map(|(ax, ay)| if ax > ay { Some(*ax) } else { None })
        .unwrap_or(aa[n - 1]);
    aa.retain(|&a| a != x);
    let rs = aa.into_iter().join(" ");
    println!("{rs}");
}
