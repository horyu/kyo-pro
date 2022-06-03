#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    let mut vvv = vec![vec![0isize; 1002]; 1002];
    for _ in 0..n {
        input! {
            lx: usize, ly: usize, rx: usize, ry: usize,
        };
        vvv[lx][ly] += 1;
        vvv[lx][ry] -= 1;
        vvv[rx][ly] -= 1;
        vvv[rx][ry] += 1;
    }
    // 二次元いもす法
    // https://imoz.jp/algorithms/imos_method.html
    for x in 0..1002 {
        for y in 1..1002 {
            vvv[x][y] += vvv[x][y - 1];
        }
    }
    for y in 0..1002 {
        for x in 1..1002 {
            vvv[x][y] += vvv[x - 1][y];
        }
    }
    let mut rs = vec![0; n + 1];
    for vv in vvv {
        for v in vv {
            rs[v as usize] += 1;
        }
    }
    println!("{}", rs[1..].iter().join("\n"));
}
