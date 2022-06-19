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
    };
    let reverse = |mut i: usize| -> usize {
        let mut tmp = 0;
        while i > 0 {
            tmp *= 10;
            tmp += i % 10;
            i /= 10;
        }
        tmp
    };
    if k % 10 == 0 || reverse(k) < k {
        println!("0");
        return;
    }
    let mut rs = 0;
    let mut kk = k;
    while kk <= n {
        rs += 1;
        kk *= 10;
    }
    let mut kk = reverse(k);
    if kk != k {
        while kk <= n {
            rs += 1;
            kk *= 10;
        }
    }
    println!("{rs}");
}
