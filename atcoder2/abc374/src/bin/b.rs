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
        s: Chars,
        t: Chars,
    };
    for (i, cs, ct) in izip!(1.., &s, &t) {
        if cs != ct {
            println!("{i}");
            return;
        }
    }
    let rs = if s.len() != t.len() {
        s.len().min(t.len()) + 1
    } else {
        0
    };
    println!("{rs}");
}
