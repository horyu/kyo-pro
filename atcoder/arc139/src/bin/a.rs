#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        tt: [usize; n]
    };
    let mut pre_a = 0usize;
    for t in tt {
        let tmp = pre_a >> t;
        for k in 0.. {
            let a = (((tmp + 1) << k) | 1) << t;
            if a > pre_a {
                pre_a = a;
                break;
            }
        }
    }
    println!("{pre_a}");
}
