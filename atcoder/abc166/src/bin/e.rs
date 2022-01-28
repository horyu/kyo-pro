#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    // j - i = ai + aj
    // j - aj = ai + i
    let mut rs = 0usize;
    let mut hm = HashMap::new();
    for (i, &a) in aa.iter().enumerate() {
        if let Some(v) = i.checked_sub(a) {
            rs += hm.get(&v).unwrap_or(&0);
        }
        *hm.entry(a + i).or_insert(0) += 1;
    }
    println!("{rs}");
}
