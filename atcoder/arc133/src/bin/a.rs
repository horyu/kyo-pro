#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut x = 0;
    for (&ax, &ay) in aa.iter().tuple_windows() {
        if ax > ay {
            x = ax;
            break;
        }
    }
    if x == 0 {
        x = aa[n - 1];
    }
    let rs = aa.into_iter().filter(|&a| a != x).join(" ");
    println!("{rs}");
}
