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
        m: usize,
        xx: [usize; n],
        ccyy: [(Usize1, usize); m],
    };
    let mut hm = HashMap::new();
    for (c, y) in ccyy {
        hm.insert(c, y);
    }
    // i 回時点で カウンタj の時の最大値を保持
    let mut vv = vec![0usize];
    let mut max = 0;
    for (i, x) in xx.into_iter().enumerate() {
        let mut new_vv = vec![max];
        for j in 0..=i {
            let mut val = x;
            if let Some(&y) = hm.get(&j) {
                val += y;
            }
            val += vv[j];
            max = max.max(val);
            new_vv.push(val);
        }
        vv = new_vv;
        // eprintln!("{}", vv.iter().join(" "));
    }
    println!("{max}");
}
