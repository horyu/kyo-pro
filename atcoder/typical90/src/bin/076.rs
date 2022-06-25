#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let sum = aa.iter().sum::<usize>();
    if sum % 10 != 0 {
        println!("No");
        return;
    }
    let aa = aa.repeat(2);
    let target = sum / 10;
    let mut crr = 0;
    let mut l = 0;
    for r in 0..(2 * n) {
        crr += aa[r];
        if crr == target {
            println!("Yes");
            return;
        }
        while l < r && target < crr {
            crr -= aa[l];
            l += 1;
        }
    }
    println!("No");
}
