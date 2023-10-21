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
        xyz: [usize; 3],
        k: usize,
    };
    let www = xyz
        .iter()
        .copied()
        .map(|n| {
            input! { mut ww: [usize; n] };
            ww.sort_unstable();
            ww.reverse();
            ww
        })
        .collect_vec();

    let f =
        |ijk: &[usize]| -> usize { izip!(&www, ijk).fold(0, |acc, (ww, &index)| acc + ww[index]) };

    let mut bh = BinaryHeap::new();
    bh.push((f(&[0; 3]), [0; 3]));
    let mut pushed = HashSet::new();
    pushed.insert([0; 3]);

    let mut rs = vec![];
    while rs.len() < k {
        let (v, mut ijk) = bh.pop().unwrap();
        rs.push(v);

        for index in 0..3 {
            ijk[index] += 1;
            if ijk[index] < xyz[index] && pushed.insert(ijk) {
                bh.push((f(&ijk), ijk));
            }
            ijk[index] -= 1;
        }
    }

    let rs = rs.iter().join("\n");
    println!("{rs}");
}
