#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        q: usize,
    };
    let mut btm = BTreeMap::new();
    for _ in 0..q {
        input! {p: u8};
        match p {
            1 => {
                input! {x: usize};
                *btm.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {x: usize, c: usize};
                if let Some(e) = btm.get_mut(&x) {
                    if *e <= c {
                        btm.remove(&x);
                    } else {
                        *e -= c;
                    }
                }
            }
            _ => {
                let rs = btm.keys().max().unwrap() - btm.keys().min().unwrap();
                println!("{rs}");
            }
        }
    }
}
