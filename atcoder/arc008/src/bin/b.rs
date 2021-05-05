#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        _m: usize,
        name: Chars,
        kit: Chars,
    };
    let mut hm_name = HashMap::new();
    for c_name in name {
        *hm_name.entry(c_name).or_insert(0) += 1;
    }
    let mut hm_kit = HashMap::new();
    for c_kit in kit {
        *hm_kit.entry(c_kit).or_insert(0) += 1;
    }
    let mut max = 0;
    for (c_name, count_name) in hm_name {
        if let Some(count_kit) = hm_kit.get(&c_name) {
            max = max.max(count_name / count_kit + (count_name % count_kit != 0) as i32);
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", max);
}
