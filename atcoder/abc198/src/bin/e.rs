#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

fn main() {
    input! {
        n: usize,
        cc: [Usize1; n],
        aabb: [(Usize1, Usize1); n - 1]
    };
    let mut vvv = vec![vec![]; n];
    for (a, b) in aabb {
        vvv[a].push(b);
        vvv[b].push(a);
    }

    let mut ttff = vec![false; n];
    let mut counts = vec![0; 1e5 as usize + 2];
    dfs(&mut ttff, &mut counts, &vvv, &cc, 0, std::usize::MAX);

    let rs = ttff
        .into_iter()
        .enumerate()
        .filter_map(|itf| if itf.1 { Some(itf.0 + 1) } else { None })
        .join("\n");
    println!("{rs}");
}

fn dfs(
    ttff: &mut Vec<bool>,
    counts: &mut Vec<i32>,
    vvv: &[Vec<usize>],
    cc: &[usize],
    crr: usize,
    par: usize,
) {
    let c = cc[crr];
    ttff[crr] = counts[c] == 0;
    counts[c] += 1;
    for &v in &vvv[crr] {
        if v != par {
            dfs(ttff, counts, vvv, cc, v, crr);
        }
    }
    counts[c] -= 1;
}
