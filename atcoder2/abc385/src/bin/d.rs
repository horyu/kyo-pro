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
        m: usize,
        mut i: isize,
        mut j: isize,
        xxyy: [(isize, isize); n],
        ddcc: [(char, isize); m],
    };
    let mut x2yy = HashMap::new();
    let mut y2xx = HashMap::new();
    for (x, y) in xxyy {
        x2yy.entry(x).or_insert(BTreeSet::new()).insert(y);
        y2xx.entry(y).or_insert(BTreeSet::new()).insert(x);
    }
    let mut cnt = 0;
    for (d, c) in ddcc {
        use std::collections::hash_map::Entry::Occupied;
        match d {
            'U' => {
                let jj = j + c;
                if let Occupied(mut o) = x2yy.entry(i) {
                    let yy = o.get_mut();
                    while let Some(&y) = yy.range(j..=jj).next() {
                        cnt += 1;
                        yy.remove(&y);
                        y2xx.get_mut(&y).unwrap().remove(&i);
                    }
                    if yy.is_empty() {
                        o.remove();
                    }
                }
                j = jj;
            }
            'D' => {
                let jj = j - c;
                if let Occupied(mut o) = x2yy.entry(i) {
                    let yy: &mut BTreeSet<isize> = o.get_mut();
                    while let Some(&y) = yy.range(jj..=j).next() {
                        cnt += 1;
                        yy.remove(&y);
                        y2xx.get_mut(&y).unwrap().remove(&i);
                    }
                    if yy.is_empty() {
                        o.remove();
                    }
                }
                j = jj;
            }
            'L' => {
                let ii = i - c;
                if let Occupied(mut o) = y2xx.entry(j) {
                    let xx = o.get_mut();
                    while let Some(&x) = xx.range(ii..=i).next() {
                        cnt += 1;
                        xx.remove(&x);
                        x2yy.get_mut(&x).unwrap().remove(&j);
                    }
                    if xx.is_empty() {
                        o.remove();
                    }
                }
                i = ii;
            }
            'R' => {
                let ii = i + c;
                if let Occupied(mut o) = y2xx.entry(j) {
                    let xx = o.get_mut();
                    while let Some(&x) = xx.range(i..=ii).next() {
                        cnt += 1;
                        xx.remove(&x);
                        x2yy.get_mut(&x).unwrap().remove(&j);
                    }
                    if xx.is_empty() {
                        o.remove();
                    }
                }
                i = ii;
            }
            _ => unreachable!(),
        }
    }
    println!("{i} {j} {cnt}");
}
