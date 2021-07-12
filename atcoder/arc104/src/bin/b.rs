#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    if n == 1 {
        println!("0");
        return;
    }
    let mut rs = 0;
    for i in 0..(n - 1) {
        let mut at = 0;
        let mut cg = 0;
        for j in i..n {
            match s[j] {
                'A' => at += 1,
                'T' => at -= 1,
                'C' => cg += 1,
                'G' => cg -= 1,
                _ => unreachable!(),
            }
            if (at == 0) && (cg == 0) {
                rs += 1;
            }
        }
    }
    println!("{}", rs);
}
