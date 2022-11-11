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
        mut aa: [usize; n],
    };
    aa.sort_unstable();
    let mut rs = 0;
    for _ in 0..2 {
        let mut bb = aa.clone().into_iter().collect::<VecDeque<usize>>();
        let mut xx = VecDeque::from(vec![bb.pop_front().unwrap()]);
        let mut odd = true;
        while !bb.is_empty() {
            let b = if odd {
                bb.pop_back().unwrap()
            } else {
                bb.pop_front().unwrap()
            };
            xx.push_front(b);
            if !bb.is_empty() {
                let b = if odd {
                    bb.pop_back().unwrap()
                } else {
                    bb.pop_front().unwrap()
                };
                xx.push_back(b);
            }
            odd = !odd;
        }

        let tmp = xx
            .into_iter()
            .tuple_windows()
            .map(|(xi, xj)| xi.abs_diff(xj))
            .sum::<usize>();
        rs = rs.max(tmp);

        aa.reverse();
    }
    println!("{rs}");
}
