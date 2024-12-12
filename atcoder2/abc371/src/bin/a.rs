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
        ab: char,
        ac: char,
        bc: char,
    };
    let rs = (0..=2)
        .permutations(3)
        .find_map(|ii| {
            let mut pos = [0; 3];
            for (i, n) in ii.iter().copied().enumerate() {
                pos[n] = i;
            }
            if ((ab == '<') == (pos[0] < pos[1]))
                && ((ac == '<') == (pos[0] < pos[2]))
                && ((bc == '<') == (pos[1] < pos[2]))
            {
                Some((b'A' + ii[1] as u8) as char)
            } else {
                None
            }
        })
        .unwrap();
    println!("{rs}");
}
