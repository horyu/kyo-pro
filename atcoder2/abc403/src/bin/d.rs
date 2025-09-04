#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        d: usize,
        aa: [usize; n],
    };
    if d == 0 {
        let rs = aa
            .into_iter()
            .counts()
            .into_values()
            .map(|c| c - 1)
            .sum::<usize>();
        println!("{rs}");
        return;
    }
    let mut dsu = ac_library::Dsu::new(n);
    let mut hm = HashMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        for k in [a.wrapping_sub(d), a, a + d] {
            if let Some(&j) = hm.get(&k) {
                dsu.merge(i, j);
            }
        }
        hm.insert(a, i);
    }
    let mut rs = 0;
    for ii in dsu.groups() {
        let cc = ii
            .into_iter()
            .map(|i| aa[i])
            .counts()
            .into_iter()
            .sorted_unstable_by_key(|kv| kv.0)
            .map(|kv| kv.1)
            .collect_vec();
        // 隣接した要素の片方が必ず0になるように任意の要素を0にする最小コストを足す
        // [直前の値が0, 直前の値が0でない]
        let mut xx = [0, 0];
        for c in cc {
            xx = [(xx[0] + c).min(xx[1] + c), xx[0]];
        }
        rs += xx[0].min(xx[1]);
    }
    println!("{rs}");
}
