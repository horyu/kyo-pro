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
        mut i: usize,
        mut j: usize,
        n: usize,
        rrcc: [(usize, usize); n],
        q: usize,
        ddll: [(char, usize); q],
    };
    let mut rows = BTreeMap::new();
    let mut cols = BTreeMap::new();
    for (r, c) in rrcc {
        rows.entry(r).or_insert_with(BTreeSet::new).insert(c);
        cols.entry(c).or_insert_with(BTreeSet::new).insert(r);
    }
    for (d, l) in ddll {
        match d {
            'L' => {
                let ll = j.saturating_sub(l).max(1);
                j = rows
                    .get(&i)
                    .and_then(|row| row.range(ll..j).max().map(|jj| jj + 1))
                    .unwrap_or(ll);
            }
            'R' => {
                let rr = (j + l).min(w);
                j = rows
                    .get(&i)
                    .and_then(|row| row.range(j..=rr).min().map(|jj| *jj - 1))
                    .unwrap_or(rr);
            }
            'U' => {
                let uu = i.saturating_sub(l).max(1);
                i = cols
                    .get(&j)
                    .and_then(|col| col.range(uu..i).max().map(|ii| ii + 1))
                    .unwrap_or(uu);
            }
            'D' => {
                let dd = (i + l).min(h);
                i = cols
                    .get(&j)
                    .and_then(|col| col.range(i..=dd).min().map(|ii| *ii - 1))
                    .unwrap_or(dd);
            }
            _ => unreachable!(),
        }
        println!("{i} {j}");
    }
}
