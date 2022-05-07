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
    // eprintln!("[{}]{pre_a}\t{:#010b}", tt[0], pre_a);
    for t in tt {
        let tmp = pre_a >> t;
        for k in 0.. {
            if (((tmp + 1) << k) | 1) << t > pre_a {
                pre_a = (((tmp + 1) << k) | 1) << t;
                break;
            }
        }
        // eprintln!("[{t}]{pre_a}\t{:#010b}", pre_a);
    }
    println!("{pre_a}");
}
