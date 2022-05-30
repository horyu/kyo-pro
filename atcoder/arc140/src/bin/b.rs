#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        _n: usize,
        cc: Chars,
    };
    // AARCC
    // ARC
    // R
    // 連続した A+の数,Rの数,C+の数
    let mut arr = [0; 3];
    // 塊の数
    let mut bts = BTreeSet::new();
    for (i, c) in cc.into_iter().enumerate() {
        match c {
            'A' => {
                if arr[1] == 0 && arr[2] == 0 {
                    arr[0] += 1;
                } else {
                    if arr[2] > 0 {
                        bts.insert((arr[0].min(arr[2]), i));
                    }
                    arr = [1, 0, 0];
                }
            }
            'R' => {
                if arr[0] > 0 && arr[1] == 0 {
                    arr[1] = 1;
                } else {
                    if arr[2] > 0 {
                        bts.insert((arr[0].min(arr[2]), i));
                    }
                    arr = [0; 3];
                }
            }
            'C' => {
                if arr[0] > 0 && arr[1] == 1 {
                    arr[2] += 1;
                } else {
                    if arr[2] > 0 {
                        bts.insert((arr[0].min(arr[2]), i));
                    }
                    arr = [0; 3];
                }
            }
            _ => {
                arr = [0; 3];
            }
        }
    }
    if arr[2] > 0 {
        bts.insert((arr[0].min(arr[2]), 0));
    }

    let mut rs = 0;
    while !bts.is_empty() {
        rs += 1;
        if rs.is_odd() {
            if let Some(&(k, i)) = bts.iter().max() {
                bts.remove(&(k, i));
                if k > 1 {
                    bts.insert((k - 1, i));
                }
            }
        } else if let Some(&(k, i)) = bts.iter().min() {
            bts.remove(&(k, i));
        }
    }
    println!("{rs}");
}
