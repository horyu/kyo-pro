#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        ddd: [[usize; n]; n],
        q: usize,
        pp: [usize; q],
    };
    let mut mm = vec![0usize];
    for size in 1..=(n * n) {
        let mut max = *mm.last().unwrap();
        for h in 1..=(size.min(n)) {
            let w = size / h;
            if size.is_multiple_of(&h) && w <= n {
                for i in 0..=(n - h) {
                    for j in 0..=(n - w) {
                        let mut tmp = 0;
                        for ii in 0..h {
                            for jj in 0..w {
                                tmp += ddd[i + ii][j + jj];
                            }
                        }
                        max = max.max(tmp);
                    }
                }
            }
        }
        mm.push(max);
    }
    for p in pp {
        println!("{}", mm[p]);
    }
}
