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
        s: Chars,
        t: Chars,
    };
    let mut vv = vec![];
    let mut j = 0;
    for sc in s {
        while sc != t[j] {
            j += 1;
        }
        j += 1;
        vv.push(j);
    }
    let rs = vv.iter().join(" ");
    println!("{rs}");
}
