#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use num_traits::Pow;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        aa: [usize; 6],
    };
    // https://atcoder.jp/contests/abc421/submissions/69008500
    let rs = f(3, 0, [0; 5], &aa, &mut HashMap::new());
    println!("{rs:.10}");
}

fn f(
    k: usize,
    l: usize,
    vv: [usize; 5],
    aa: &[usize],
    memo: &mut HashMap<(usize, usize, [usize; 5]), f64>,
) -> f64 {
    if let Some(&rs) = memo.get(&(k, l, vv)) {
        return rs;
    }
    let rest = 5 - l;
    if k == 0 || rest == 0 {
        let rs = vv
            .into_iter()
            .map(|v| aa[v])
            .sorted_unstable()
            .dedup_with_count()
            .map(|(cnt, a)| cnt * a)
            .max()
            .unwrap() as f64;
        memo.insert((k, l, vv), rs);
        return rs;
    }

    let mut sum = 0f64;
    for pp in (0..rest).map(|_| 0..6).multi_cartesian_product() {
        let mut tpm = 0f64;
        let mut ww = vv;
        let iter = if k == 1 { rest..=rest } else { 0..=rest };
        for size in iter {
            for qq in pp.iter().copied().combinations(size) {
                ww[l..(l + size)].copy_from_slice(&qq);
                tpm = tpm.max(f(k - 1, l + size, ww, aa, memo));
            }
        }
        sum += tpm;
    }
    let rs = sum / 6f64.powi(rest as i32);
    memo.insert((k, l, vv), rs);
    rs
}
