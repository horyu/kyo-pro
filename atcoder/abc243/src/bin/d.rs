#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        _n: usize,
        x: usize,
        cc: Chars
    };
    let mut vv = VecDeque::new();
    for c in cc {
        if vv.is_empty() {
            vv.push_back(c);
            continue;
        }
        if c == 'U' && *vv.back().unwrap() != 'U' {
            vv.pop_back();
        } else {
            vv.push_back(c);
        }
    }
    let mut rs = x;
    for c in vv {
        rs = match c {
            'U' => rs / 2,
            'L' => rs * 2,
            _ => rs * 2 + 1,
        };
    }
    println!("{rs}");
}
