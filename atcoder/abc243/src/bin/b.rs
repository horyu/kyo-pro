#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    let mut ahm = HashMap::new();
    for (i, a) in aa.into_iter().enumerate() {
        ahm.insert(a, i);
    }
    let mut rs1 = 0;
    let mut rs2 = 0;
    for (j, b) in bb.into_iter().enumerate() {
        if let Some(&i) = ahm.get(&b) {
            if i == j {
                rs1 += 1;
            } else {
                rs2 += 1;
            }
        }
    }
    println!("{rs1}\n{rs2}",);
}
