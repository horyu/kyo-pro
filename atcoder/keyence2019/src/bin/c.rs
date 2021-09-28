#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [isize; n],
        bb: [isize; n]
    };
    let mut pp = vec![];
    let mut nn = vec![];
    for (a, b) in aa.into_iter().zip(bb.into_iter()) {
        let d = a - b;
        use std::cmp::Ordering;
        match d.cmp(&0) {
            Ordering::Greater => pp.push(d),
            Ordering::Less => nn.push(d),
            _ => (),
        }
    }
    let n_sum = nn.iter().sum::<isize>();
    if n_sum == 0 {
        println!("0");
        return;
    }
    pp.sort_unstable();
    let mut p_sum = 0;
    let mut p_cnt = 0;
    while let Some(p) = pp.pop() {
        p_sum += p;
        p_cnt += 1;
        if p_sum + n_sum >= 0 {
            println!("{}", p_cnt + nn.len());
            return;
        }
    }
    println!("-1");
}
