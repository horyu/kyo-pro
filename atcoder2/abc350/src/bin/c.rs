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
        mut i2a: [Usize1; n],
    };
    let mut a2i = vec![0; n];
    for (i, a) in i2a.iter().copied().enumerate() {
        a2i[a] = i;
    }
    let mut rrss = vec![];
    for a in 0..n {
        if a2i[a] == a {
            continue;
        }
        let i = a2i[a];
        rrss.push((a + 1, i + 1));
        let b = i2a[a];
        i2a.swap(a, i);
        a2i.swap(a, b);
        // eprintln!("a:{a} i:{i} b:{b} i2a:{i2a:?} a2i:{a2i:?}");
    }
    // eprintln!("{i2a:?}");
    // eprintln!("{a2i:?}");
    println!("{}", rrss.len());
    for (i, j) in rrss {
        println!("{i} {j}");
    }
}
