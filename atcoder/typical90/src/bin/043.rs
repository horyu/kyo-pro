#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: Usize1,
        cs: Usize1,
        rt: Usize1,
        ct: Usize1,
        sss: [Chars; h]
    };
    // 01-BFS 01BFS
    // https://betrue12.hateblo.jp/entry/2018/12/08/000020
    let sss = sss
        .into_iter()
        .map(|ss| ss.into_iter().map(|s| s == '.').collect_vec())
        .collect_vec();
    let mut vvv = vec![vec![std::usize::MAX; w]; h];
    let mut viewed = vec![vec![(false, false, false, false); w]; h];
    let mut qq = VecDeque::new();
    vvv[rs][cs] = 0;
    viewed[rs][cs] = (true, true, true, true);
    qq.push_back((rs, cs));
    while let Some((r, c)) = qq.pop_front() {
        let v = vvv[r][c] + 1;
        for rr in (0..r).rev() {
            if !sss[rr][c] || viewed[rr][c].0 {
                break;
            }
            viewed[rr][c].0 = true;
            if v < vvv[rr][c] {
                vvv[rr][c] = v;
                qq.push_back((rr, c));
            }
        }
        for rr in (r + 1)..h {
            if !sss[rr][c] || viewed[rr][c].1 {
                break;
            }
            viewed[rr][c].1 = true;
            if v < vvv[rr][c] {
                vvv[rr][c] = v;
                qq.push_back((rr, c));
            }
        }
        for cc in (0..c).rev() {
            if !sss[r][cc] || viewed[r][cc].2 {
                break;
            }
            viewed[r][cc].2 = true;
            if v < vvv[r][cc] {
                vvv[r][cc] = v;
                qq.push_back((r, cc));
            }
        }
        for cc in (c + 1)..w {
            if !sss[r][cc] || viewed[r][cc].3 {
                break;
            }
            viewed[r][cc].3 = true;
            if v < vvv[r][cc] {
                vvv[r][cc] = v;
                qq.push_back((r, cc));
            }
        }
    }
    println!("{}", vvv[rt][ct] - 1);
}
