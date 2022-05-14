#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    // input! {
    //     _w: usize
    // };
    let mut vv = vec![];
    for i in 1..=99 {
        vv.push(i);
        vv.push(i * 100);
        vv.push(i * 10000);
    }
    println!("{}\n{}", vv.len(), vv.into_iter().join(" "));
}
