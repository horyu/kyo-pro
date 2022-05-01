#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        rr: [usize; n],
        cc: [usize; n],
        q: usize,
        rrcc: [(Usize1, Usize1); q],
    };
    let mut rs = String::new();
    for (r, c) in rrcc {
        let b = rr[r].max(cc[c]);
        let s = rr[r].min(cc[c]);
        if n - s >= b {
            rs.push('.');
        } else {
            rs.push('#');
        }
    }
    println!("{}", rs);
}
