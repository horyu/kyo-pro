#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut ss: Chars
    };
    let mut hm = HashMap::new();
    for &s in &ss {
        *hm.entry(s).or_insert(0) += 1;
    }
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        let sl = ss[l];
        if let Some(x) = hm.get_mut(&sl) {
            *x -= 1;
            if *x == 0 {
                hm.remove(&sl);
            }
        }
        let min = *hm.keys().min().unwrap();
        while ss[l] > min && l < r {
            let sr = ss[r];
            if let Some(x) = hm.get_mut(&sr) {
                *x -= 1;
                if *x == 0 {
                    hm.remove(&sr);
                }
            }
            if sr == min {
                ss.swap(l, r);
            }
            r -= 1;
        }
        l += 1;
    }
    let rs = ss.iter().join("");
    println!("{rs}");
}
