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
        // hhwwdd: [(usize, usize, usize); n],
        hhwwdd: [[usize; 3]; n],
    };
    let x2i = hhwwdd
        .iter()
        .flatten()
        .copied()
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<usize, usize>>();
    // dbg!(&x2i);
    let aabbcc = hhwwdd
        .into_iter()
        .map(|vv| {
            vv.into_iter()
                .map(|v| x2i.get(&v).copied().unwrap())
                .sorted_unstable()
                .collect_vec()
        })
        .sorted_unstable()
        .collect_vec();
    // for abc in aabbcc {
    //     eprintln!("{abc:?}");
    // }
    let mut btm: BTreeMap<usize, usize> = BTreeMap::new();
    for (_, aabbcc) in aabbcc
        .into_iter()
        .into_group_map_by(|abc| abc[0])
        .into_iter()
        .sorted_unstable_by_key(|x| x.0)
    {
        for abc in aabbcc.iter() {
            let [a, b, c] = abc[..] else {return;};
            if let Some((&bb, &cc)) = btm.range(..b).next_back() {
                if cc < c {
                    println!("Yes");
                    return;
                }
            }
        }
        for abc in aabbcc {
            let [a, b, c] = abc[..] else {return;};
            let mut remove_bb = vec![];
            for (&bb, &cc) in btm.range(b..) {
                if c <= cc {
                    remove_bb.push(bb);
                } else {
                    break;
                }
            }
            for bb in remove_bb {
                btm.remove(&bb);
            }

            let e = btm.entry(b).or_insert(c);
            *e = (*e).min(c);
        }
    }
    println!("No");
}
