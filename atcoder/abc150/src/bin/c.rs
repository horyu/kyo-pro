#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use std::ops::Sub;

use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        pp: [usize; n],
        qq: [usize; n],
    };
    let mut p_num = -1;
    let mut q_num = -1;
    for (num, v) in (1..=n).permutations(n).enumerate() {
        if (p_num < 0) && (v == pp) {
            p_num = num as isize;
        }
        if (q_num < 0) && (v == qq) {
            q_num = num as isize;
        }
        if (p_num >= 0) && (q_num >= 0) {
            break;
        }
    }
    println!("{}", p_num.sub(q_num).abs());
}
