#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aabb: [(usize, usize); n],
    };
    let mut ab_to_i = HashMap::new();
    let mut i_to_ab = HashMap::new();
    let mut j = 0usize;
    for &(a, b) in &aabb {
        if let std::collections::hash_map::Entry::Vacant(e) = ab_to_i.entry(a) {
            e.insert(j);
            i_to_ab.insert(j, a);
            j += 1;
        }
        if let std::collections::hash_map::Entry::Vacant(e) = ab_to_i.entry(b) {
            e.insert(j);
            i_to_ab.insert(j, b);
            j += 1;
        }
    }
    let Some(&one_key) = ab_to_i.get(&1) else {
        println!("1");
        return;
    };
    // dbg!(one_key);
    let mut dsu = ac_library_rs::Dsu::new(ab_to_i.len());
    for &(a, b) in &aabb {
        let aa = *ab_to_i.get(&a).unwrap();
        let bb = *ab_to_i.get(&b).unwrap();
        // eprintln!("{a}-{aa}:{b}-{bb}");
        dsu.merge(aa, bb);
    }
    if let Some(gg) = dsu.groups().into_iter().find(|g| g.contains(&one_key)) {
        let rs = gg
            .into_iter()
            .map(|g| i_to_ab.get(&g).unwrap())
            .max()
            .unwrap();
        println!("{rs}");
    } else {
        panic!("?");
    }
}
