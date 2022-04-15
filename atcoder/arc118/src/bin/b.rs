#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        k: usize,
        n: usize,
        m: usize,
        aa: [usize; k]
    };
    // b/M = a/N, bN=aM
    // b = aM/N
    let mut iibbdd = aa
        .iter()
        .enumerate()
        .map(|(i, &a)| {
            let f = (a * m) as f64 / n as f64;
            (i, f.floor() as usize, f - f.floor())
        })
        .sorted_by(|x, y| x.2.partial_cmp(&y.2).unwrap())
        .collect_vec();
    let sum = iibbdd.iter().map(|ibd| ibd.1).sum::<usize>();
    iibbdd
        .iter_mut()
        .rev()
        .take(m - sum)
        .for_each(|ibd| ibd.1 += 1);
    let mut rs = vec![0; k];
    for (i, b, _d) in iibbdd {
        rs[i] = b;
    }
    println!("{}", rs.into_iter().join(" "));
}
