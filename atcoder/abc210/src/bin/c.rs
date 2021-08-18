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
        cc: [usize; n],
    };
    let mut hm = HashMap::new();
    let mut cnt = 0;
    for i in 0..k {
        if let Some(v) = hm.get_mut(&cc[i]) {
            *v += 1;
        } else {
            cnt += 1;
            hm.insert(cc[i], 1);
        }
    }
    let mut rs = cnt;
    for i in k..n {
        if let Some(v) = hm.get_mut(&cc[i - k]) {
            *v -= 1;
            if *v == 0 {
                cnt -= 1;
            }
        }
        if let Some(v) = hm.get_mut(&cc[i]) {
            *v += 1;
            if *v == 1 {
                cnt += 1;
            }
        } else {
            cnt += 1;
            hm.insert(cc[i], 1);
        }
        rs = rs.max(cnt);
    }
    println!("{}", rs);
}
