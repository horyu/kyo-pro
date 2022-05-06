#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [u8; n]
    };
    let mut aa: VecDeque<_> = aa.into_iter().collect();
    let mut rev = false;
    while let Some(a) = aa.pop_front() {
        if !rev {
            if a == 0 {
                rev = !rev;
            } else {
                println!("No");
                return;
            }
            while let Some(&0) = aa.back() {
                aa.pop_back();
            }
        } else {
            if a == 1 {
                rev = !rev;
            } else {
                println!("No");
                return;
            }
            while let Some(&1) = aa.back() {
                aa.pop_back();
            }
        }
    }
    println!("Yes");
}
