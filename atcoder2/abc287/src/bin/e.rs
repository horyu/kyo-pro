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
        ss: [Bytes; n],
    };
    let mut rs = vec![0; n];
    let iiss = ss.iter().map(|s| s.as_slice()).enumerate().collect_vec();
    dfs(&mut rs, 0, &iiss);
    let rs = rs.iter().join("\n");
    println!("{rs}");
}

fn dfs(rs: &mut [i32], depth: i32, iiss: &[(usize, &[u8])]) {
    let mut vvv = vec![vec![]; 26];
    for (i, s) in iiss.iter().copied() {
        if s.is_empty() {
            rs[i] = depth;
            continue;
        }
        vvv[(s[0] - b'a') as usize].push((i, &s[1..]));
    }
    for vv in vvv {
        if vv.len() <= 1 {
            for is in vv {
                rs[is.0] = depth;
            }
            continue;
        }
        dfs(rs, depth + 1, &vv);
    }
}
