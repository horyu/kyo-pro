#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    if n == 1 {
        println!("{}", aa[0]);
        return;
    }
    let rs = aa[1..].iter().fold(aa[0], |acc, a| acc.gcd(a));
    println!("{}", rs);
}
