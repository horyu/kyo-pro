#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [Chars; h]
    };
    let mut cnt = 0;
    for i in 0..(h - 1) {
        for j in 0..w {
            if ss[i][j] == '.' && ss[i + 1][j] == '.' {
                cnt += 1;
            }
        }
    }
    #[allow(clippy::needless_range_loop)]
    for i in 0..h {
        for j in 0..(w - 1) {
            if ss[i][j] == '.' && ss[i][j + 1] == '.' {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
