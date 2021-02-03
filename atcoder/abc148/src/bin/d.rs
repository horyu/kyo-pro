#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use std::collections::VecDeque;

// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let mut aa: VecDeque<_> = aa.into_iter().collect();
    let mut next = 1;
    let mut breaks = 0;
    while !aa.is_empty() {
        let a = aa.pop_front().unwrap();
        if next == a {
            next += 1;
        } else {
            breaks += 1;
        }
    }
    if next == 1 {
        println!("-1");
    } else {
        println!("{}", breaks);
    }
}
