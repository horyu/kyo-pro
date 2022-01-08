#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut k: usize,
    };
    let mut v = vec![];
    let mut exp = 63u32;
    while exp != 0 {
        let pow = 2usize.pow(exp);
        if k >= pow {
            v.push(2);
            k -= pow;
        } else {
            v.push(0);
        }
        exp -= 1;
    }
    if k == 1 {
        v.push(2);
    } else {
        v.push(0);
    }
    let rs = v[v.iter().position(|&x| x == 2).unwrap()..].iter().join("");
    println!("{}", rs);
}
