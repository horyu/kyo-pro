#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut ccc: [Chars; 8]
    };
    let mut yyxx = vec![];
    for (i, cc) in ccc.iter().enumerate() {
        for (j, c) in cc.iter().enumerate() {
            if *c == 'Q' {
                yyxx.push((i as isize, j as isize));
            }
        }
    }
    if is_valid(&yyxx[..2]) && is_valid(&yyxx[..3]) && dfs(&mut yyxx) {
        for (y, x) in yyxx {
            ccc[y as usize][x as usize] = 'Q';
        }
        for cc in ccc {
            println!("{}", cc.iter().join(""));
        }
    } else {
        println!("No Answer");
    }
}

fn is_valid(yyxx: &[(isize, isize)]) -> bool {
    let len = yyxx.len();
    let (y, x) = yyxx[len -1];
    yyxx[0..(len - 1)].iter().all(|&(yy, xx)| {
        x != xx && y != yy && ((x - xx).abs() != (y - yy).abs())
    })
}

fn dfs(yyxx: &mut Vec<(isize, isize)>) -> bool {
    if yyxx.len() == 8 {
        return true;
    }
    for i in 0..8 {
        for j in 0..8 {
            yyxx.push((i, j));
            if is_valid(yyxx) && dfs(yyxx) {
                return true;
            }
            yyxx.pop();
        }
    }
    false
}
