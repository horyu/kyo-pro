#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut wwvv: [(usize, usize); n],
        xx: [usize; m],
        llrr: [(Usize1, Usize1); q]
    };
    // 高価値・大きい順に並べる
    wwvv.sort_unstable_by(|x, y| y.1.cmp(&x.1).then(y.0.cmp(&x.0)));
    // 小さい順の箱のインデックス
    let mut order_to_i = vec![0usize; m];
    for (j, i) in (0..m).sorted_by_key(|&i| xx[i]).enumerate() {
        order_to_i[j] = i;
    }
    for (l, r) in llrr {
        let mut used = vec![false; m];
        for i in l..=r {
            used[i] = true;
        }
        let mut rs = 0usize;
        for &(w, v) in &wwvv {
            for i in 0..m {
                if used[order_to_i[i]] {
                    continue;
                }
                if w <= xx[order_to_i[i]] {
                    used[order_to_i[i]] = true;
                    rs += v;
                    break;
                }
            }
        }
        println!("{rs}");
    }
}
