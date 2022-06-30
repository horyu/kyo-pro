#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
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
    let mut oo = BTreeSet::new();
    let mut xx = BTreeSet::new();
    for (i, &c) in s.iter().enumerate() {
        if c == 'o' {
            oo.insert(i);
        } else {
            xx.insert(i);
        }
    }
    let mut rs = 0usize;
    for i in 0..n {
        let other = if s[i] == 'o' { &xx } else { &oo };
        if let Some(j) = other.range(i..).next() {
            rs += n - j;
        }
    }
    println!("{rs}");
}
