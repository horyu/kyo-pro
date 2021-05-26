#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        aaa: [[usize; 4]; 4]
    };
    let tf = (0..4).any(|i| {
        (0..4).any(|j| {
            if (i != 0) && (aaa[i - 1][j] == aaa[i][j]) {
                return true;
            }
            if (j != 0) && (aaa[i][j - 1] == aaa[i][j]) {
                return true;
            }
            false
        })
    });
    println!("{}", ["GAMEOVER", "CONTINUE"][tf as usize]);
}
