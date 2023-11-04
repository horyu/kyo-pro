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
        aa: [Usize1; n],
    };
    let mut cc = vec![0; 1e5 as usize + 1];
    // 尺取り方
    let mut rs = 0;
    let mut l = 0;
    for r in 0..n {
        cc[aa[r]] += 1;
        while 1 < cc[aa[r]] {
            cc[aa[l]] -= 1;
            l += 1;
        }
        rs = rs.max(r - l + 1);
    }

    println!("{rs}");
}
