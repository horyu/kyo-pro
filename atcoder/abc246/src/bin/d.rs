#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize
    };
    let tri = n.nth_root(3u32);
    let a_max = if tri.pow(3) < n { tri + 1 } else { tri };
    let mut b = 0usize;
    let mut rs = a_max.pow(3);
    for a in ((a_max / 2)..a_max).rev() {
        let aa = a * a;
        let aaa = aa * a;
        let sum = loop {
            let bb = b * b;
            let bbb = bb * b;
            let sum = aaa + aa * b + a * bb + bbb;
            if sum >= n {
                break sum;
            }
            b += 1;
        };
        rs = rs.min(sum);
    }
    println!("{rs}");
}
