#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aaa: [[Usize1; n - 1]; n],
    };
    let mut vv = aaa.iter().map(|aa| aa[0]).collect_vec();
    let mut cc = vec![0; n];
    let mut ii = (0..n).collect_vec();
    let mut rs = 0;
    let mut cnt = 0;
    while cnt < n * (n - 1) / 2 {
        rs += 1;
        let mut updated = HashSet::new();
        let mut next_ii = vec![];
        for i in ii {
            let vi = vv[i];
            if cc[i] == n - 1 || updated.contains(&i) || updated.contains(&vi) {
                continue;
            }
            if vv[vi] == i {
                cnt += 1;

                updated.insert(i);
                updated.insert(vi);
                // eprintln!("{rs} {i} {} {} {}", vi, vv[vi], cc[i]);

                cc[i] += 1;
                cc[vi] += 1;

                if cc[i] < n - 1 {
                    vv[i] = aaa[i][cc[i]];
                    next_ii.push(i);
                }
                if cc[vi] < n - 1 {
                    vv[vi] = aaa[vi][cc[vi]];
                    next_ii.push(vi);
                }
            }
        }
        if updated.is_empty() {
            // eprintln!("{}", vv.iter().join(","));
            // eprintln!("{}", cc.iter().join(","));
            println!("-1");
            return;
        }
        ii = next_ii;
    }

    println!("{rs}");
}
