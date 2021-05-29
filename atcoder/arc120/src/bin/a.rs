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
    let mut max = aa[0];
    let mut a_sum = 0;
    let mut wa = 0;
    for (i, a) in aa.into_iter().enumerate() {
        max = max.max(a);
        a_sum += a;
        wa += a_sum;
        println!("{}", wa + max * (i + 1));
    }
}
