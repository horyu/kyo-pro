#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        hh: [usize; n],
        aabb: [(Usize1, Usize1); m]
    };
    let mut v = vec![true; n];
    for (a, b) in aabb {
        use std::cmp::Ordering;
        match hh[a].cmp(&hh[b]) {
            Ordering::Equal => {
                v[a] = false;
                v[b] = false;
            }
            Ordering::Greater => {
                v[b] = false;
            }
            Ordering::Less => {
                v[a] = false;
            }
        }
    }
    println!("{}", v.iter().filter(|&&x| x).count());
}
