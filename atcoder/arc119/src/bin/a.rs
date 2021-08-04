#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize
    };
    if n < 3 {
        println!("{}", n);
        return;
    }
    let mut rs = std::usize::MAX;
    for b in 1..std::usize::MAX {
        let b_pow = 2usize.pow(b as u32);
        if b_pow > n {
            break;
        }
        let (a, c) = n.div_rem(&b_pow);
        rs = rs.min(a + b + c);
    }
    println!("{}", rs);
}
