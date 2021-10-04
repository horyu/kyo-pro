#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut k: usize,
        a: usize,
        b: usize,
    };
    // 叩き続けた場合
    let mut max = k + 1;
    // 交換できるだけ交換して1回余ったら叩く
    if (a < b) && (a < k) {
        k -= a + 1;
        let n = b + (b - a) * (k / 2) + k % 2;
        max = max.max(n);
    }
    println!("{}", max);
}
