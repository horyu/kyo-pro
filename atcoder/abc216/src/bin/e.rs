#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };
    if aa.iter().sum::<usize>() <= k {
        let rs = aa.into_iter().map(|a| a * (a + 1) / 2).sum::<usize>();
        println!("{rs}");
        return;
    }

    let mut l = 1;
    let mut r = 3e9 as usize;
    let is_ok = |x| aa.iter().map(|&a| (a + 1).saturating_sub(x)).sum::<usize>() <= k;
    while r - l > 1 {
        let mid = (r + l) / 2;
        if is_ok(mid) {
            r = mid;
        } else {
            l = mid;
        }
    }
    let mut rs = 0;
    let aa = aa.into_iter().filter(|&a| a >= l).collect_vec();
    rs += aa.iter().map(|&a| (l + a) * (a + 1 - l) / 2).sum::<usize>();
    let used = aa.iter().map(|&a| a.saturating_sub(l - 1)).sum::<usize>();
    rs -= used.saturating_sub(k) * l;
    println!("{rs}");
}
