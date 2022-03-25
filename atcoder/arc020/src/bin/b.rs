#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        c: usize,
        aa: [usize; n]
    };
    let mut hhmm = vec![HashMap::new(); 2];
    for (i, a) in aa.into_iter().enumerate() {
        *hhmm[i % 2].entry(a).or_insert(0) += 1;
    }
    let sum = hhmm
        .iter()
        .fold(0, |acc, hm| acc + hm.values().sum::<usize>());
    let rs = (1..=10)
        .tuple_combinations()
        .map(|(i, j)| {
            let ij = sum - hhmm[0].get(&i).unwrap_or(&0) - hhmm[1].get(&j).unwrap_or(&0);
            let ji = sum - hhmm[0].get(&j).unwrap_or(&0) - hhmm[1].get(&i).unwrap_or(&0);
            c * ij.min(ji)
        })
        .min()
        .unwrap();
    println!("{rs}");
}
