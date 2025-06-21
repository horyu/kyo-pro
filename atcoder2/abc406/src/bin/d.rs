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
        n: usize,
        xxyy: [(usize, usize); n],
        q: usize,
        ttvv: [(usize, usize); q],
    };
    let mut i2jj: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut j2ii: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (x, y) in xxyy {
        i2jj.entry(x).or_default().insert(y);
        j2ii.entry(y).or_default().insert(x);
    }
    for (t, v) in ttvv {
        let mut rs = 0;
        if t == 1 {
            // 行削除
            if let Some(jj) = i2jj.remove(&v) {
                rs = jj.len();
                for j in jj {
                    if let Some(ii) = j2ii.get_mut(&j) {
                        ii.remove(&v);
                        if ii.is_empty() {
                            j2ii.remove(&j);
                        }
                    }
                }
            }
        } else {
            // 列削除
            if let Some(ii) = j2ii.remove(&v) {
                rs = ii.len();
                for i in ii {
                    if let Some(jj) = i2jj.get_mut(&i) {
                        jj.remove(&v);
                        if jj.is_empty() {
                            i2jj.remove(&i);
                        }
                    }
                }
            }
        }
        println!("{rs}");
    }
}
