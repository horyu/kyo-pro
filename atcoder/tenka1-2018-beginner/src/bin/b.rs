#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        k: usize
    };
    let mut x = &mut a;
    let mut y = &mut b;
    for _ in 0..k {
        if x.is_odd() {
            *x -= 1
        }
        *y += *x / 2;
        *x /= 2;
        std::mem::swap(&mut x, &mut y)
    }
    println!("{} {}", a, b);
}
