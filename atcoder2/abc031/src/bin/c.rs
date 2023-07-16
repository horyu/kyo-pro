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
        aa: [isize; n],
    };
    let mut rs = isize::MIN;
    for i in 0..n {
        let mut xx = isize::MIN;
        let mut yy = isize::MIN;
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut x = 0;
            let mut y = 0;
            for (idx, a) in aa[i.min(j)..=i.max(j)].iter().copied().enumerate() {
                if idx.is_even() {
                    x += a;
                } else {
                    y += a;
                }
            }
            if yy < y {
                yy = y;
                xx = x;
            }
        }
        rs = rs.max(xx);
    }
    println!("{rs}");
}
