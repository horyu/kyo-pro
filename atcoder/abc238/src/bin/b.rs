#![allow(clippy::many_single_char_names)]
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
    let mut vv = vec![false; 361];
    vv[0] = true;
    vv[360] = true;
    let mut crr = 0;
    for a in aa {
        crr += a;
        crr %= 360;
        vv[crr] = true;
    }
    let rs = vv
        .into_iter()
        .enumerate()
        .filter_map(|iv| if iv.1 { Some(iv.0) } else { None })
        .tuple_windows()
        .map(|(x, y)| y - x)
        .max()
        .unwrap();
    println!("{rs}");
}
