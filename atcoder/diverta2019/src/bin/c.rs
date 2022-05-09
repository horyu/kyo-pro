#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    };
    let (mut l, mut c, mut r, mut lr) = (0, 0, 0, 0);
    for s in ss {
        let b = s.starts_with('B');
        let a = s.ends_with('A');
        if b {
            l += 1;
        }
        if a {
            r += 1;
        }
        if b && a {
            lr += 1;
        }
        c += s.matches("AB").count();
    }
    let mut rs = c + l.min(r);
    if lr > 0 && lr == l && l == r {
        rs -= 1;
    }
    println!("{rs}");
}
