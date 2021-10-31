#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: u128,
        aa: [u128; n],
        x: u128
    };
    let a_sum = aa.iter().sum::<u128>();
    let mut cnt = x / a_sum * n;
    let mut sum = x / a_sum * a_sum;
    for &a in &aa {
        if sum > x {
            break;
        }
        cnt += 1;
        sum += a;
    }
    println!("{}", cnt);
}
