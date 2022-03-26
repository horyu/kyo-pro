#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: isize,
        aa: [isize; n],
        bb: [isize; n],
    };
    let mut hs = HashSet::from_iter([aa[0], bb[0]]);
    for (&a, &b) in aa[1..].iter().zip(bb[1..].iter()) {
        let xx = if a == b { vec![a] } else { vec![a, b] };
        let mut new_hs = HashSet::new();
        for v in hs {
            for &x in &xx {
                if (v - x).abs() <= k {
                    new_hs.insert(x);
                }
            }
        }
        if new_hs.is_empty() {
            println!("No");
            return;
        }
        hs = new_hs;
    }
    println!("Yes");
}
