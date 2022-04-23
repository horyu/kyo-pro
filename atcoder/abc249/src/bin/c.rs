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
        ss: [Chars; n]
    };
    let vvv = ss
        .into_iter()
        .map(|s| {
            let mut vv = [0; 26];
            for c in s {
                vv[(c as u8 - b'a') as usize] += 1;
            }
            vv
        })
        .collect_vec();
    let mut rs = 0;
    for size in 1..=n {
        for vvv in vvv.iter().combinations(size) {
            let mut arr = [0; 26];
            for vv in vvv {
                for (i, c) in vv.iter().enumerate() {
                    arr[i] += c;
                }
            }
            rs = rs.max(arr.into_iter().filter(|&x| x == k).count());
        }
    }
    println!("{rs}");
}
