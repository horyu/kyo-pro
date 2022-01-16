#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{fastout, input, marker::*};
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [usize; n],
        xxkk: [(usize, Usize1); q],
    };
    let mut hm = HashMap::new();
    for (i, a) in aa.into_iter().enumerate() {
        let v = hm.entry(a).or_insert_with(Vec::new);
        v.push(i + 1);
    }
    for (x, k) in xxkk {
        if let Some(v) = hm.get(&x) {
            if let Some(i) = v.get(k) {
                println!("{}", i);
                continue;
            }
        }
        println!("-1");
    }
}
