#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        aa: [Usize1; n]
    };
    let mut viewed = HashSet::new();
    let mut next = x;
    loop {
        if viewed.contains(&next) {
            break;
        }
        viewed.insert(next);
        next = aa[next];
    }
    println!("{}", viewed.len());
}
