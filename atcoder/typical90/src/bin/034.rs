#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };
    let mut hm = HashMap::new();
    let mut l = 0;
    let mut rs = 0;
    for r in 0..n {
        *hm.entry(aa[r]).or_insert(0) += 1;
        while hm.len() > k {
            let al = aa[l];
            if let Some(e) = hm.get_mut(&al) {
                if *e == 1 {
                    hm.remove(&al);
                } else {
                    *e -= 1;
                }
            }
            l += 1;
        }
        rs = rs.max(r - l + 1);
    }
    println!("{rs}");
}
