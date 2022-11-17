#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n],
    };
    let mut hs = HashSet::new();
    for ((ix, iy), (jx, jy)) in xxyy.into_iter().tuple_combinations() {
        let dx = ix - jx;
        let dy = iy - jy;
        let gcd = dx.abs().gcd(&dy.abs());
        let sign = if 0 <= dx { 1 } else { -1 };
        let x = sign * dx / gcd;
        let y = sign * dy / gcd;
        hs.insert((x, y));
        hs.insert((-x, -y));
    }
    println!("{}", hs.len());
}
