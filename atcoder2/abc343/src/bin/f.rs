#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut aa: [usize; n],
        ttxxyy: [(usize, usize, usize); q],
    };
    let sqrt_n = n.sqrt();
    let mut bbttmm = vec![BTreeMap::<usize, usize>::default(); sqrt_n + 2];
    for (i, a) in aa.iter().copied().enumerate() {
        *bbttmm[i / sqrt_n].entry(a).or_default() += 1usize;
    }

    for (t, x, y) in ttxxyy {
        if t == 1 {
            let p = x - 1;
            // 減らす
            let old_a = aa[p];
            let e = bbttmm[p / sqrt_n].get_mut(&old_a).unwrap();
            *e -= 1;
            if *e == 0 {
                bbttmm[p / sqrt_n].remove(&old_a);
            }
            // 増やす
            let new_a = y;
            aa[p] = new_a;
            *bbttmm[p / sqrt_n].entry(new_a).or_default() += 1;
            continue;
        }

        let l = x - 1;
        let r = y - 1;
        // aa[l..=r] における2番目に大きい値の個数を求める
        let mut btm = BTreeMap::<usize, usize>::new();
        let mut i = l;
        while i <= r {
            if i % sqrt_n == 0 && i + sqrt_n - 1 <= r {
                // 区間の左端が sqrt_n で割り切れる場合
                for (&k, &cnt) in bbttmm[i / sqrt_n].iter().rev().take(2) {
                    *btm.entry(k).or_default() += cnt;
                }
                i += sqrt_n;
            } else {
                *btm.entry(aa[i]).or_default() += 1;
                i += 1;
            }
        }
        let rs = btm.into_iter().rev().nth(1).unwrap_or_default().1;
        println!("{rs}");
    }
}
