#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut vv = vec![];
    for (k, g) in aa.into_iter().group_by(|&a| a).into_iter() {
        vv.push((k, g.count()));
    }
    let mut sum = 0;
    let mut zeros = 0;
    let mut zero_range = 0usize;
    let mut ones = 0;
    let mut one_range = 0;
    for (a, size) in vv {
        if a == 1 {
            sum += size;

            zero_range = zero_range.saturating_sub(size);

            one_range += size;
            ones = ones.max(one_range);
        } else {
            zero_range += size;
            zeros = zeros.max(zero_range);

            one_range = one_range.saturating_sub(size);
        }
    }
    let min = sum - ones;
    let max = sum + zeros;
    // dbg!(sum, one_range, zero_range);
    // eprintln!("{min} {max}");
    let rs = max - min + 1;
    println!("{rs}");
}
