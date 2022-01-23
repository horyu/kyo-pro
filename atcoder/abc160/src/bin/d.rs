#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: isize,
        x: Isize1,
        y: Isize1,
    };
    let mut rs = vec![0; 2 * 1000 + 1];
    for i in 0..n {
        for j in (i + 1)..n {
            let k = (j - i).abs()
                .min((x - i).abs() + (y - j).abs() + 1)
                .min((y - i).abs() + (x - j).abs() + 1);
            rs[k as usize] += 1;
        }
    }
    println!("{}", rs[1..(n as usize)].iter().join("\n"));
}
