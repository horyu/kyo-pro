#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use std::usize;

// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aappxx: [(usize, usize, usize) ; n],
    };
    let mut price = std::usize::MAX;
    for (a, p, x) in aappxx {
        if a < x {
            price = price.min(p);
        }
    }
    if price == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", price);
    }
}
