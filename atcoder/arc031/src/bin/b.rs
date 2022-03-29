#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
       aaa: [Chars; 10]
    };
    // 0未チェック/海 1~島番号
    let mut map = [[0; 10]; 10];
    let mut num = 0;
    for i in 0..10 {
        for j in 0..10 {
            if aaa[i][j] == 'x' || map[i][j] != 0 {
                continue;
            }
            num += 1;
            let mut qq = VecDeque::new();
            map[i][j] = num;
            qq.push_back((i, j));
            while let Some((qy, qx)) = qq.pop_front() {
                for (y, x) in udlr(qy, qx, 9, 9) {
                    if map[y][x] == 0 && aaa[y][x] == 'o' {
                        map[y][x] = num;
                        qq.push_back((y, x));
                    }
                }
            }
        }
    }
    for i in 0..10 {
        for j in 0..10 {
            if aaa[i][j] == 'x' {
                let mut ttff = vec![false; num];
                for (y, x) in udlr(i, j, 9, 9) {
                    if let Some(n) = map[y][x].checked_sub(1) {
                        ttff[n] = true;
                    }
                }
                if ttff.into_iter().all(|tf| tf) {
                    println!("YES");
                    return;
                }
            }
        }
    }
    println!("NO");
}

fn udlr(y: usize, x: usize, y_max: usize, x_max: usize) -> Vec<(usize, usize)> {
    let mut vv = Vec::with_capacity(4);
    if y > 0 {
        vv.push((y - 1, x));
    }
    if y < y_max {
        vv.push((y + 1, x));
    }
    if x > 0 {
        vv.push((y, x - 1));
    }
    if x < x_max {
        vv.push((y, x + 1));
    }
    vv
}
