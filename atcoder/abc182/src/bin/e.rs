#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
#![allow(clippy::needless_range_loop)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); n],
        ccdd: [(Usize1, Usize1); m],
    };
    // 0: 未解明 1: 光 2: 岩 3: ランプ
    let mut vvv = vec![vec![0; w]; h];
    for (c, d) in ccdd {
        vvv[c][d] = 2;
    }
    for (a, b) in aabb {
        vvv[a][b] = 3;
        for j in (0..b).rev() {
            if vvv[a][j] >= 2 {
                break;
            }
            vvv[a][j] = 1;
        }
        for j in (b + 1)..w {
            if vvv[a][j] >= 2 {
                break;
            }
            vvv[a][j] = 1;
        }
        for i in (0..a).rev() {
            if vvv[i][b] >= 2 {
                break;
            }
            vvv[i][b] = 1;
        }
        for i in (a + 1)..h {
            if vvv[i][b] >= 2 {
                break;
            }
            vvv[i][b] = 1;
        }
    }

    let rs = vvv.into_iter().fold(0, |acc, vv| {
        acc + vv.into_iter().filter(|&v| v == 1 || v == 3).count()
    });
    println!("{rs}");
}
