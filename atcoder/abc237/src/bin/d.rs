#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut qq = VecDeque::new();
    qq.push_back(n);
    for (i, c) in s.into_iter().enumerate().rev() {
        if c == 'L' {
            qq.push_back(i);
        } else {
            qq.push_front(i);
        }
    }
    println!("{}", qq.iter().join(" "));
}
