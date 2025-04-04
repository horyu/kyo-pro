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
        ccc: [Chars; n],
    };
    let mut aaa = vec![vec![!0usize; n]; n];
    let mut qq = VecDeque::new();
    for i in 0..n {
        qq.push_back((i, i));
        aaa[i][i] = 0;
    }
    for (i, cc) in ccc.iter().enumerate() {
        for (j, c) in cc.iter().copied().enumerate() {
            if i == j || c == '-' {
                continue;
            }
            qq.push_back((i, j));
            aaa[i][j] = 1;
        }
    }
    while let Some((i, j)) = qq.pop_front() {
        for k in 0..n {
            for l in 0..n {
                // k-i ~ j-l
                if ccc[k][i] != '-' && ccc[j][l] != '-' && ccc[k][i] == ccc[j][l] && aaa[k][l] == !0
                {
                    aaa[k][l] = aaa[i][j] + 2;
                    qq.push_back((k, l));
                }
            }
        }
    }
    for aa in aaa {
        let rs = aa
            .into_iter()
            .map(|a| if a == !0 { -1 } else { a as isize })
            .join(" ");
        println!("{rs}");
    }
}
