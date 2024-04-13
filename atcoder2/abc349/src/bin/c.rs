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
    let t = t.into_iter().map(|c| c.to_ascii_lowercase()).collect_vec();
    let mut mm = multimap::MultiMap::new();
    for (i, c) in s.iter().copied().enumerate() {
        mm.insert(c, i);
    }
    if let (Some(v0), Some(v1), Some(v2)) =
        (mm.get_vec(&t[0]), mm.get_vec(&t[1]), mm.get_vec(&t[2]))
    {
        let i0 = v0[0];
        if let Some(&i1) = v1.iter().find(|&&i| i0 < i) {
            if let Some(&i2) = v2.iter().find(|&&i| i1 < i) {
                println!("Yes");
                return;
            }
        }
    }
    if t[2] == 'x' {
        if let (Some(v0), Some(v1)) = (mm.get_vec(&t[0]), mm.get_vec(&t[1])) {
            let i0 = v0[0];
            if let Some(&i1) = v1.iter().find(|&&i| i0 < i) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
