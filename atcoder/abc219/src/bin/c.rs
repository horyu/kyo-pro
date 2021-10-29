#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: Chars,
        n: usize,
        mut ss: [String; n]
    };
    ss.sort_by_cached_key(|s| {
        s.chars()
            .map(|c| x.iter().position(|&xc| xc == c).unwrap())
            .collect_vec()
    });
    for s in ss {
        println!("{}", s);
    }
}
