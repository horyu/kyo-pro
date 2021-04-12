#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: Chars
    };
    let n = n.iter().map(|&c| c.to_digit(10).unwrap() % 3).collect_vec();
    let ones = n.iter().filter(|&&i| i == 1).count();
    let twos = n.iter().filter(|&&i| i == 2).count();
    let k = n.len();
    let sum = n.iter().sum::<u32>();
    let rs = match sum % 3 {
        0 => 0,
        1 => {
            if (ones >= 1) && (k > 1) {
                1
            } else if (twos >= 2) && (k > 2) {
                2
            } else {
                -1
            }
        }
        2 => {
            if (twos >= 1) && (k > 1) {
                1
            } else if (ones >= 2) && (k > 2) {
                2
            } else {
                -1
            }
        }
        _ => unreachable!(),
    };
    println!("{}", rs);
}
