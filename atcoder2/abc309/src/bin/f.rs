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
        // hhwwdd: [(usize, usize, usize); n],
        hhwwdd: [[usize; 3]; n],
    };
    let aabbcc = hhwwdd
        .into_iter()
        .map(|mut vv| -> [usize; 3] {
            vv.sort_unstable();
            vv.try_into().unwrap()
        })
        .sorted_unstable_by_key(|abc| (abc[0], std::cmp::Reverse(abc[1])))
        .collect_vec();
    // for abc in aabbcc {
    //     eprintln!("{abc:?}");
    // }
    let mut btm: BTreeMap<usize, usize> = BTreeMap::new();
    for [a, b, c] in aabbcc {
        if let Some((&_bb, &cc)) = btm.range(..b).next_back() {
            if cc < c {
                println!("Yes");
                return;
            }
        }
        let mut remove_bb = vec![];
        for (&bb, &cc) in btm.range(b..) {
            if c <= cc {
                remove_bb.push(bb);
            } else {
                break;
            }
        }
        for bb in remove_bb {
            btm.remove(&bb);
        }

        let e = btm.entry(b).or_insert(c);
        *e = (*e).min(c);
    }
    println!("No");
}
