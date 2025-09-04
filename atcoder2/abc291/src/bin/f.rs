#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        m: usize,
        ss: [Chars; n],
    };
    let mut rg = vec![vec![]; n];
    let mut lg = vec![vec![]; n];
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.iter().copied().enumerate() {
            if c == '1' {
                rg[i].push(i + 1 + j);
                lg[i + 1 + j].push(i);
            }
        }
    }
    // rd[i] = 都市0から都市iまでの最小移動回数
    let mut rd = vec![!0; n];
    rd[0] = 0;
    for (i, jj) in rg.iter().enumerate() {
        for j in jj.iter().copied() {
            if rd[i] != !0 {
                rd[j] = rd[j].min(rd[i] + 1);
            }
        }
    }
    // ld[i] = 都市n-1から都市iまでの最小移動回数
    let mut ld = vec![!0; n];
    ld[n - 1] = 0;
    for (i, jj) in lg.iter().enumerate().rev() {
        for j in jj.iter().copied() {
            if ld[i] != !0 {
                ld[j] = ld[j].min(ld[i] + 1);
            }
        }
    }
    // 都市kを経由しない場合の都市0から都市n-1までの最小移動回数
    let mut rrss = vec![-1; n - 2];
    for k in 1..(n - 1) {
        let mut rs = !0usize;
        for i in (k.saturating_sub(m - 1))..k {
            if rd[i] == !0 {
                continue;
            }
            for &j in rg[i].iter() {
                if k < j && ld[j] != !0 {
                    rs = rs.min(rd[i] + ld[j] + 1);
                }
            }
        }

        if rs != !0 {
            rrss[k - 1] = rs as i32;
        }
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
