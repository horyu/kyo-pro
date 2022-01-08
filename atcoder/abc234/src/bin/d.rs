#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{fastout, input, marker::*};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        pp: [usize; n]
    };
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut bh = BinaryHeap::new();
    for &p in &pp[0..k] {
        bh.push(Reverse(p));
    }
    let mut min = bh.pop().unwrap().0;
    println!("{}", min);
    for &p in &pp[k..n] {
        if p > min {
            bh.push(Reverse(p));
            min = bh.pop().unwrap().0;
        }
        println!("{}", min);
    }
}
