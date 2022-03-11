#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize
    };
    // 1a

    // 1aa
    // 2ab

    // 1aaa
    // 2aab,aba,abb
    // 3abc

    // 1aaaa
    // 2aaab,aaba,aabb,abaa,abab,abba,abbb
    // 3aabc,abac,abbc,abca,abcb,abcc
    // 4abcd

    let mut vvv = vec![(1, vec![0])];
    for _ in 1..n {
        let mut next_vvv = vec![];
        for (i, vv) in vvv {
            for j in 0..i {
                let mut tmp = vv.clone();
                tmp.push(j);
                next_vvv.push((i, tmp));
            }
            let mut tmp = vv.clone();
            tmp.push(i);
            next_vvv.push((i + 1, tmp));
        }
        vvv = next_vvv;
    }
    for (_i, vv) in vvv {
        // eprintln!("{i}:{}", vv.into_iter().map(|v| (b'a' + v) as char).join(""));
        println!("{}", vv.into_iter().map(|v| (b'a' + v) as char).join(""));
    }
}
