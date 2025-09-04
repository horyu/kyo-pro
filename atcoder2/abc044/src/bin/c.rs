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
        a: isize,
        xx: [isize; n],
    };
    let mut hm = HashMap::new();
    for mut x in xx {
        x -= a;
        let mut new_hm = hm.clone();
        *new_hm.entry((1, x)).or_insert(0) += 1usize;
        for ((i, v), cnt) in hm {
            *new_hm.entry((i + 1, v + x)).or_insert(0) += cnt;
        }
        hm = new_hm;
    }
    let rs = hm
        .into_iter()
        .filter_map(|((_i, v), cnt)| if v == 0 { Some(cnt) } else { None })
        .sum::<usize>();
    println!("{rs}");
}
