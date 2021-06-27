#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ttllrr: [(usize, usize, usize); n]
    };
    let rs = ttllrr
        .iter()
        .tuple_combinations()
        .filter(|&(a, b)| range_contain(a, b))
        .count();

    println!("{}", rs);
}

fn range_contain(x: &(usize, usize, usize), y: &(usize, usize, usize)) -> bool {
    if x.1 == y.1 {
        return true;
    }
    let (l, r) = if x.1 < y.1 { (x, y) } else { (y, x) };
    use std::cmp::Ordering;
    match l.2.cmp(&r.1) {
        Ordering::Less => false,
        Ordering::Equal => [1, 3].contains(&l.0) && [1, 2].contains(&r.0),
        Ordering::Greater => true,
    }
}
