#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [Chars; h]
    };
    let ccc = ss
        .into_iter()
        .map(|s| s.into_iter().map(|c| c == '.').collect_vec())
        .collect_vec();
    let mut rs = 0;

    for y in 0..h {
        for x in 0..w {
            if !ccc[y][x] {
                continue;
            }
            let mut cnt = 0;
            let mut qq = VecDeque::new();
            let mut pushed = vec![vec![false; w]; h];
            qq.push_back((y, x));
            pushed[y][x] = true;

            while !qq.is_empty() {
                rs = rs.max(cnt);
                let mut new_qq = VecDeque::new();
                while let Some((qy, qx)) = qq.pop_front() {
                    for (i, j) in udlr(qy, qx, h, w) {
                        if pushed[i][j] || !ccc[i][j] {
                            continue;
                        }

                        pushed[i][j] = true;
                        new_qq.push_back((i, j));
                    }
                }
                cnt += 1;
                qq = new_qq;
            }
        }
    }

    println!("{rs}");
}

fn udlr(y: usize, x: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut vv = Vec::with_capacity(2);
    if 0 < y {
        vv.push((y - 1, x));
    }
    if y < h - 1 {
        vv.push((y + 1, x));
    }
    if 0 < x {
        vv.push((y, x - 1));
    }
    if x < w - 1 {
        vv.push((y, x + 1));
    }
    vv
}
