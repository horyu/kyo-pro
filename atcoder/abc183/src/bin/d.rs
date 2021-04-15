#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        w: isize,
        ssttpp: [(usize, usize, isize); n]
    };
    let mut vv = vec![0isize; 2 * 10usize.pow(5) + 1];
    for (s, t, p) in ssttpp {
        vv[s] += p;
        vv[t] -= p;
    }
    let mut amount = 0isize;
    for v in vv {
        amount += v;
        if amount > w {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
