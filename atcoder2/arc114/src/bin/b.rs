#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
        ff: [Usize1; n],
    };
    let mut dsu = ac_library::Dsu::new(n);
    for (i, f) in ff.into_iter().enumerate() {
        dsu.merge(i, f);
    }
    let mut rs = ModInt998244353::new(1);
    let size = dsu.groups().len();
    if 1 < size {
        rs += 1;
        rs = rs.pow(size as u64) - 1;
    }
    println!("{rs}");
}
