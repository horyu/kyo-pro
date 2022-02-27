#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
        bb: [usize; n],
    };
    for i in 0..(n - 2) {
        let b = bb[i];
        if aa[i] == b {
            continue;
        }
        if let Some(tmp) = aa[i..].iter().position(|&a| a == b) {
            let mut j = i + tmp;
            while j >= i + 2 {
                aa.swap(j - 2, j);
                aa.swap(j - 1, j);
                j -= 2;
            }
            if j == i + 1 {
                aa.swap(i, i + 1);
                aa.swap(i + 1, i + 2);
            }
        } else {
            println!("No");
            return;
        }
    }
    let mut tf = aa == bb;
    if !tf
        && aa
            .iter()
            .sorted()
            .group_by(|&&a| a)
            .into_iter()
            .any(|(_, g)| g.count() >= 2)
    {
        let cc = aa[(n - 2)..].iter().sorted().collect_vec();
        let dd = bb[(n - 2)..].iter().sorted().collect_vec();
        tf = cc == dd;
    }
    println!("{}", ["No", "Yes"][tf as usize]);
}
