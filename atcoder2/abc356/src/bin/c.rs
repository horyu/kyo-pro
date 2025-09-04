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
        k: usize,
    };
    let mut aaa = vec![];
    let mut rr = vec![];
    for _ in 0..m {
        input! {
            c: usize,
            aa: [Usize1; c],
            r: char,
        };
        aaa.push(aa);
        rr.push(r == 'o');
    }
    let mut rs = 0usize;
    for bits in 0usize..(1 << n) {
        if izip!(&aaa, &rr).all(|(aa, &r)| {
            let mut cnt = 0;
            for a in aa.iter().copied() {
                if bits & (1 << a) != 0 {
                    cnt += 1;
                }
            }
            if r {
                k <= cnt
            } else {
                cnt < k
            }
        }) {
            rs += 1;
        }
    }
    println!("{rs}");
}
