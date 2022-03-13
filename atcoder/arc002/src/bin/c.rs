#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        s: String
    };
    let rs = (0..4)
        .map(|_| ['A', 'B', 'Y', 'X'])
        .multi_cartesian_product()
        .map(|cc| {
            let l = cc[..2].iter().join("");
            let r = cc[2..].iter().join("");
            s.replace(&l, "L").replace(&r, "R").len()
        })
        .min()
        .unwrap();

    println!("{rs}");
}
