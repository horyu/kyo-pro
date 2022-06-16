#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        q: usize,
        ttxx: [(usize, usize); q]
    };
    let mut qq = VecDeque::new();
    for (t, x) in ttxx {
        match t {
            1 => {
                qq.push_front(x);
            }
            2 => {
                qq.push_back(x);
            }
            _ => {
                println!("{}", qq[x - 1]);
            }
        }
    }
}
