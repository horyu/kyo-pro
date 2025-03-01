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
        mut aa: [usize; n],
    };
    let mut imos = vec![0isize; n + 1];
    let mut tmp = 0;
    let mut rrss = vec![];
    for (i, mut a) in aa.iter().copied().enumerate() {
        tmp += imos[i];
        a += tmp as usize;
        imos[i + 1] += 1;
        imos[n.min(i + 1 + a)] -= 1;
        rrss.push(a.saturating_sub(n - 1 - i));
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
