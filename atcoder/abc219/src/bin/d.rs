#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        aabb: [(usize, usize); n]
    };
    let mut hm: HashMap<(usize, usize), i32> = HashMap::new();
    for (a, b) in aabb {
        let mut new_hm = hm.clone();
        for ((p, q), c) in hm {
            let pp = (p + a).min(x);
            let qq = (q + b).min(y);
            if let Some(e) = new_hm.get_mut(&(pp, qq)) {
                *e = (*e).min(c + 1);
            } else {
                new_hm.insert((pp, qq), c + 1);
            }
        }
        new_hm.insert((a.min(x), b.min(y)), 1);
        hm = new_hm;
    }
    let rs = hm.get(&(x, y)).unwrap_or(&-1);
    println!("{rs}");
}
