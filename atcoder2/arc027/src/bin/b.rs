#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s1: Chars,
        s2: Chars,
    };
    let c2i = chain!(&s1, &s2)
        .copied()
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<HashMap<_, _>>();
    let i2c = c2i.iter().map(|(c, i)| (*i, *c)).collect::<HashMap<_, _>>();
    let mut dsu = ac_library::Dsu::new(c2i.len());
    for (ci, c2) in izip!(&s1, &s2) {
        let i1 = c2i[ci];
        let i2 = c2i[c2];
        dsu.merge(i1, i2);
    }
    if !('0'..='9')
        .filter_map(|c| c2i.get(&c).copied())
        .map(|i| dsu.leader(i))
        .all_unique()
    {
        println!("0");
        return;
    }
    let mut rs = 1usize;
    for gg in dsu.groups() {
        let cc = gg.iter().map(|&i| i2c[&i]).collect_vec();
        // eprintln!("{cc:?}");
        if cc.iter().copied().any(|c| c.is_ascii_digit()) {
            continue;
        }
        if cc.contains(&s1[0]) || cc.contains(&s2[0]) {
            rs *= 9;
        } else {
            rs *= 10;
        }
    }
    println!("{rs}");
}
