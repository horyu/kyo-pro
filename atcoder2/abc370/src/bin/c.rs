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
        mut s: Chars,
        t: Chars,
    };
    let n = s.len();
    let mut rrss = vec![];
    while s != t {
        let mut vv = vec![];
        for (i, (sc, tc)) in izip!(s.iter().copied(), t.iter().copied()).enumerate() {
            if sc != tc {
                let mut v = s.clone();
                v[i] = tc;
                vv.push(v);
            }
        }
        let min = vv.into_iter().min().unwrap();
        rrss.push(min.clone());
        s = min;
    }
    println!("{}", rrss.len());
    for rs in rrss {
        println!("{}", rs.iter().join(""));
    }
}
