#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use proconio::{input, marker::*};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        bb: [Usize1; n - 1]
    };
    let mut hm = HashMap::new();
    hm.insert(0, Vec::new());
    for (i, &b) in bb.iter().enumerate() {
        (*hm.entry(b).or_insert(Vec::new())).push(i + 1);
    }
    println!("{}", calc_pay(0, &hm));
}

fn calc_pay(i: usize, hm: &HashMap<usize, Vec<usize>>) -> usize {
    dbg!(i, hm.get(&i));
    if let Some(juniors) = hm.get(&i) {
        let minmax = juniors.iter().map(|&junior| calc_pay(junior, &hm)).minmax();
        match minmax {
            MinMaxResult::MinMax(min, max) => min + max + 1,
            MinMaxResult::OneElement(one) => 2 * one + 1,
            _ => unreachable!(),
        }
    } else {
        1
    }
}
