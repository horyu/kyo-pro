#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let mut r_sum = aa[0];
    let mut l_sum = aa[n - 1];
    let mut r = 1;
    let mut l = n - 2;
    while r <= l {
        if r_sum <= l_sum {
            r_sum += aa[r];
            r += 1;
        } else {
            l_sum += aa[l];
            l -= 1;
        }
    }
    println!("{}", r_sum.max(l_sum) - r_sum.min(l_sum));
}
