#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        mut v: usize,
        xx: [usize; 3]
    };
    let x_sum = xx.iter().sum::<usize>();
    v %= x_sum;
    let rs = if v < xx[0] {
        'F'
    } else {
        v -= xx[0];
        if v < xx[1] {
            'M'
        } else {
            'T'
        }
    };
    println!("{rs}");
}
