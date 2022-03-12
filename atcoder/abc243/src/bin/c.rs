#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n],
        s: Chars
    };
    let mut rhm = HashMap::new();
    let mut lhm = HashMap::new();
    for (i, (x, y)) in xxyy.into_iter().enumerate() {
        if s[i] == 'R' {
            let e = rhm.entry(y).or_insert(std::isize::MAX);
            *e = x.min(*e);
        } else {
            let e = lhm.entry(y).or_insert(std::isize::MIN);
            *e = x.max(*e);
        }
    }
    for (y, r) in rhm {
        if let Some(&l) = lhm.get(&y) {
            if r < l {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
