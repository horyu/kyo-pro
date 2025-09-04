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
        l: isize,
        r: isize,
        aa: [isize; n],
    };
    let sum = aa.iter().sum::<isize>();
    let mut rs = sum;
    let mut btm = BTreeMap::new();
    let mut ltmp = 0;
    for (i, &a) in aa.iter().enumerate() {
        ltmp += l - a;
        btm.entry(ltmp).or_insert_with(Vec::new).push(i);
        rs = rs.min(sum + ltmp);
    }

    let mut rtmp = 0;
    for (i, &a) in aa.iter().enumerate().rev() {
        match btm.entry(ltmp) {
            std::collections::btree_map::Entry::Occupied(mut e) => {
                let vv = e.get_mut();
                vv.pop();
                if vv.is_empty() {
                    e.remove_entry();
                }
            }
            std::collections::btree_map::Entry::Vacant(_) => unreachable!(),
        }
        ltmp -= l - a;

        rtmp += r - a;
        rs = rs.min(sum + rtmp);
        if let Some((k, v)) = btm.iter().min() {
            rs = rs.min(sum + rtmp + btm.iter().min().unwrap().0);
        }
    }

    println!("{rs}");
}
