#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n]
    };
    if n == 1 {
        println!("1");
        return;
    }
    let mut hm = HashMap::new();
    for (a, b) in xxyy.iter().tuple_combinations() {
        let mut dx = b.0 - a.0;
        let mut dy = b.1 - a.1;
        match dx.cmp(&0) {
            std::cmp::Ordering::Less => {
                dx *= -1;
                dy *= -1;
            }
            std::cmp::Ordering::Equal => {
                dy = dy.abs();
            }
            std::cmp::Ordering::Greater => (),
        }
        *hm.entry((dx, dy)).or_insert(0) += 1;
    }
    if let Some(c) = hm.values().max() {
        println!("{}", n - c);
    }
}
