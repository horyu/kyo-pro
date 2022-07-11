#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    };
    let mut rrss = vec![0; n];
    // https://qiita.com/python_walker/items/d1e2be789f6e7a0851e5
    // 最長増加部分列(LIS)
    for tf in [false, true] {
        if tf {
            aa.reverse();
        }
        let mut vv = vec![];
        for (i, &a) in aa.iter().enumerate() {
            if *vv.last().unwrap_or(&0) < a {
                vv.push(a);
            } else {
                let j = vv.partition_point(|&v| v < a);
                vv[j] = a;
            }
            rrss[if tf { n - 1 - i } else { i }] += vv.len();
        }
    }
    let rs = rrss.into_iter().max().unwrap() - 1;
    println!("{rs}");
}
