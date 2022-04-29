#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: usize,
        q: usize,
        ttnncc: [(u8, Usize1, Usize1); q]
    };
    let mut rs = vec![0; c];
    let mut yoko = HashMap::new();
    let mut tate = HashMap::new();
    for (t, n, c) in ttnncc.into_iter().rev() {
        if t == 1 {
            if yoko.insert(n, c).is_none() {
                rs[c] += w - tate.len();
            }
        } else if tate.insert(n, c).is_none() {
            rs[c] += h - yoko.len();
        }
    }
    println!("{}", rs.into_iter().join(" "));
}
