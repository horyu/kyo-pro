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
        q: usize,
        mut s: Chars,
        xxcc: [(Usize1, char); q],
    };
    let mut ttff = vec![false; n - 2];
    let mut rs = 0;
    for (i, (c0, c1, c2)) in s.iter().copied().tuple_windows().enumerate() {
        if (c0, c1, c2) == ('A', 'B', 'C') {
            ttff[i] = true;
            rs += 1;
        }
    }
    for (x, c) in xxcc {
        s[x] = c;
        for i in x.saturating_sub(2)..=(x).min(n - 3) {
            if ttff[i] {
                ttff[i] = false;
                rs -= 1;
            }
            if (s[i], s[i + 1], s[i + 2]) == ('A', 'B', 'C') {
                ttff[i] = true;
                rs += 1;
            }
        }
        println!("{rs}");
    }
}
