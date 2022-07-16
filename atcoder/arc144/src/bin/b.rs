#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_roundings)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        a: isize,
        b: isize,
        vv: [isize; n]
    };
    let mut l = *vv.iter().min().unwrap();
    let mut r = *vv.iter().max().unwrap() + 1;
    while 1 < (r - l) {
        let mid = (l + r) / 2;
        let mut tmp = 0;
        for &v in &vv {
            if mid <= v {
                tmp += (v - mid) / b;
            } else {
                tmp -= (mid - v).div_ceil(a);
            }
        }
        if 0 <= tmp {
            l = mid;
        } else {
            r = mid;
        }
    }
    println!("{l}");
}
