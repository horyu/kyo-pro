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
        mut s1: Chars,
        mut s2: Chars,
    };
    let i2c = chain!(&s1, &s2)
        .copied()
        .unique()
        .enumerate()
        .collect::<HashMap<_, _>>();
    let c2i = i2c.iter().map(|(&i, &c)| (c, i)).collect::<HashMap<_, _>>();
    let mut dsu = ac_library::Dsu::new(c2i.len());
    for (c1, c2) in izip!(&s1, &s2) {
        dsu.merge(c2i[c1], c2i[c2]);
    }
    let mut rs = 1usize;
    for vv in dsu.groups() {
        if vv.iter().any(|v| i2c[v].is_ascii_digit()) {
            continue;
        }
        if dsu.same(vv[0], c2i[&s1[0]]) || dsu.same(vv[0], c2i[&s2[0]]) {
            rs *= 9;
        } else {
            rs *= 10;
        }
    }
    println!("{rs}");
}
