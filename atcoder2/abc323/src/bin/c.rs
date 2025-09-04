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
        aa: [usize; m],
        ss: [Chars; n],
    };
    let mut cc = (1..=n).collect_vec();
    let mut bbb = vec![BinaryHeap::new(); n];
    for (si, s) in ss.into_iter().enumerate() {
        for (i, c) in s.into_iter().enumerate() {
            let a = aa[i];
            if c == 'o' {
                cc[si] += a;
            } else {
                bbb[si].push(a);
            }
        }
    }
    for (i, mut c) in cc.iter().copied().enumerate() {
        let mut max_c = 0;
        for (j, cj) in cc.iter().copied().enumerate() {
            if i != j {
                max_c = max_c.max(cj);
            }
        }
        let mut rs = 0;
        while c < max_c {
            c += bbb[i].pop().unwrap();
            rs += 1;
        }
        println!("{rs}");
    }
}
