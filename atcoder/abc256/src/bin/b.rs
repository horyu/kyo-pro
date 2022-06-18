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
    let mut p = 0;
    let mut vv = VecDeque::new();
    for a in aa {
        vv.push_back(0);
        for v in vv.iter_mut() {
            *v += a;
        }
        while !vv.is_empty() && vv[0] > 3 {
            p += 1;
            vv.pop_front();
        }
    }
    println!("{p}");
}
