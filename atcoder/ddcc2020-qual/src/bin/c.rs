#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
#![allow(clippy::needless_range_loop)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        _k: usize,
        sss: [Chars; h]
    };
    let mut rs = vec![vec![0; w]; h];
    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if sss[i][j] == '#' {
                cnt += 1;
                rs[i][j] = cnt;
            }
        }
    }
    for i in 0..h {
        for j in 1..w {
            if rs[i][j - 1] != 0 && rs[i][j] == 0 {
                rs[i][j] = rs[i][j - 1];
            }
        }
        for j in (1..w).rev() {
            if rs[i][j - 1] == 0 && rs[i][j] != 0 {
                rs[i][j - 1] = rs[i][j];
            }
        }
    }
    for j in 0..w {
        for i in 1..h {
            if rs[i - 1][j] != 0 && rs[i][j] == 0 {
                rs[i][j] = rs[i - 1][j];
            }
        }
        for i in (1..h).rev() {
            if rs[i - 1][j] == 0 && rs[i][j] != 0 {
                rs[i - 1][j] = rs[i][j];
            }
        }
    }

    for r in rs {
        println!("{}", r.into_iter().join(" "));
    }
}
