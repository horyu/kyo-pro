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
        h: usize,
        w: usize,
        aaa: [[usize; w]; h],
    };
    let mut rs = 0;
    for vv in (0..(h + w - 2))
        .map(|_| [false, true])
        .multi_cartesian_product()
    {
        if vv.iter().filter(|&&tf| tf).count() != h - 1 {
            continue;
        }
        let mut x = 0;
        let mut y = 0;
        let mut ww = vec![aaa[0][0], aaa[h - 1][w - 1]];
        for tf in vv {
            if tf {
                x += 1;
            } else {
                y += 1;
            }
            ww.push(aaa[x][y]);
        }
        ww.sort_unstable();
        ww.dedup();
        if ww.len() == h + w - 1 {
            rs += 1;
        }
    }
    println!("{rs}");
}
