#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    };
    let mut r1 = 0;
    let mut r2 = 0;
    let mut t1 = 0;
    let mut t2 = 0;
    for _ in 0..x {
        if t1 < a {
            r1 += b;
        }
        t1 += 1;
        if t1 == a + c {
            t1 = 0;
        }

        if t2 < d {
            r2 += e;
        }
        t2 += 1;
        if t2 == d + f {
            t2 = 0;
        }
    }
    let rs = match r1.cmp(&r2) {
        std::cmp::Ordering::Less => "Aoki",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Takahashi",
    };
    println!("{rs}");
}
