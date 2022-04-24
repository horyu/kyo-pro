#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut rs = vec![];
    for (x, y, z) in std::iter::once(0)
        .chain(aa)
        .chain(std::iter::once(2e9 as usize))
        .tuple_windows()
    {
        if (x <= y && y > z) || (x > y && y <= z) {
            rs.push(1);
        } else {
            rs.push(0);
        }
    }
    let rs = rs.into_iter().join(" ");
    println!("{rs}");
}
