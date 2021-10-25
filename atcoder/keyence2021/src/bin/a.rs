#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [u128; n],
        bb: [u128; n],
    };
    let mut max = aa[0] * bb[0];
    let mut amax = 0;
    for (a, b) in aa.into_iter().zip(bb) {
        amax = amax.max(a);
        max = max.max(amax * b);
        println!("{}", max);
    }
}
