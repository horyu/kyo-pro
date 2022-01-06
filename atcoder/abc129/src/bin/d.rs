#![allow(unused_imports)]
use ascii::{AsciiChar, AsciiStr};
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [Chars; h]
    };
    let sss = ss
        .into_iter()
        .map(|s| s.into_iter().map(|c| c == '.').collect_vec())
        .collect_vec();
    // [y][x] にランプを置いたとき そのマスを除いて[UDLR] の照らせる数
    let mut vvv = vec![vec![[0; 4]; w]; h];
    // U D
    for i in 1..h {
        for j in 0..w {
            if sss[i - 1][j] {
                vvv[i][j][0] = vvv[i - 1][j][0] + 1;
            }
            if sss[h - i][j] {
                vvv[h - i - 1][j][1] = vvv[h - i][j][1] + 1;
            }
        }
    }
    // L R
    for j in 1..w {
        for i in 0..h {
            if sss[i][j - 1] {
                vvv[i][j][2] = vvv[i][j - 1][2] + 1;
            }
            if sss[i][w - j] {
                vvv[i][w - j - 1][3] = vvv[i][w - j][3] + 1;
            }
        }
    }
    let mut rs = 0;
    for i in 0..h {
        for j in 0..w {
            if sss[i][j] {
                rs = rs.max(1 + vvv[i][j].iter().sum::<usize>());
            }
        }
    }
    println!("{}", rs);
}
