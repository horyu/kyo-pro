#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: isize,
    };
    // k(k+1)/2 >= n
    // kk + k - 2n >= 0
    for k in ((-1 + (1 + 4 * 2 * n).sqrt()) / 2)..std::isize::MAX {
        if k * k + k >= 2 * n {
            println!("{}", k);
            return;
        }
    }
}
