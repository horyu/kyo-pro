#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        x: usize,
        mut aa: [usize; n]
    };
    for a in aa.iter_mut() {
        if *a >= x {
            let used = (*a / x).min(k);
            *a -= used * x;
            k -= used;
        }
        if k == 0 {
            break;
        }
    }
    aa.sort_unstable();
    for a in aa.iter_mut().rev() {
        if *a > 0 && k > 0 {
            *a = 0;
            k -= 1;
            if k > 0 {
                continue;
            }
        }
        break;
    }
    let rs = aa.into_iter().sum::<usize>();
    println!("{rs}");
}
