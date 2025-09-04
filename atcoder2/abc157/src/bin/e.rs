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
        n: usize,
        s: Bytes,
        q: usize,
    };
    let mut vv = s.into_iter().map(|b| (b - b'a') as usize).collect_vec();
    let mut bbttss = vec![BTreeSet::default(); 26];
    for (i, v) in vv.iter().copied().enumerate() {
        bbttss[v].insert(i);
    }
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {i: Usize1, c: char};
            let b = (c as u8 - b'a') as usize;
            if vv[i] != b {
                bbttss[vv[i]].remove(&i);
                bbttss[b].insert(i);
                vv[i] = b;
            }
            continue;
        }
        input! {l: Usize1, r: Usize1};
        let mut rs = 0;
        for bts in bbttss.iter() {
            if bts.range(l..=r).next().is_some() {
                rs += 1;
            }
        }
        println!("{rs}");
    }
}
