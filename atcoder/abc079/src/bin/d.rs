#![allow(unused_imports)]
use std::collections::VecDeque;

// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut ccc: [[usize; 10]; 10],
        aaa: [[isize; w]; h]
    };
    // ワーシャルフロイド法
    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                ccc[i][j] = ccc[i][j].min(ccc[i][k] + ccc[k][j]);
            }
        }
    }

    let mut rs = 0;
    for aa in aaa {
        for a in aa {
            if a != -1 {
                rs += ccc[a as usize][1];
            }
        }
    }
    println!("{}", rs);
}
