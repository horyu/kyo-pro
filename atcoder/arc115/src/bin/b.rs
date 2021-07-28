#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        ccc: [[isize; n]; n]
    };
    if n == 1 {
        println!("Yes\n{}\n0", ccc[0][0]);
        return;
    }
    let out = (0..n).chain(0..=0).tuple_combinations().any(|(i, j)| {
        (1..n).any(|m| {
            (ccc[i][0] - ccc[j][0] != ccc[i][m] - ccc[j][m])
                && (ccc[0][i] - ccc[0][j] != ccc[m][i] - ccc[m][j])
        })
    });
    if out {
        println!("No");
        return;
    }
    // a1 = A とすると b_1 = c_11 - A
    // a_i - A = c_i1 - c_11   => a_i = c_i1 - C_11 + A
    // b_j - b_1 = c_1j - c_11 => b_j = c_1j - A
    // 1 <= i,j <= n にたいして 0 <= a_i, b_j とできるようなAを見つけられたらOK
    // println!("Yes\n{}\n{}");
}
