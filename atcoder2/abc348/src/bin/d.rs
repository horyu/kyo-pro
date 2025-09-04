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
        h: usize,
        w: usize,
        aaa: [Chars; h],
        n: usize,
        rrccee: [(Usize1, Usize1, usize); n],
    };
    let mut s = (0, 0);
    let mut t = (0, 0);
    for (i, aa) in aaa.iter().enumerate() {
        for (j, &a) in aa.iter().enumerate() {
            match a {
                'S' => s = (i, j),
                'T' => t = (i, j),
                _ => {}
            }
        }
    }
    let mut hm = HashMap::new();
    for (r, c, e) in rrccee {
        hm.insert((r, c), e);
    }

    let mut bh = BinaryHeap::new();
    bh.push((0, s));

    let mut ddd = vec![vec![0; w]; h];
    while let Some((mut qe, (qi, qj))) = bh.pop() {
        if (qi, qj) == t {
            println!("Yes");
            return;
        }
        if let Some(&e) = hm.get(&(qi, qj)) {
            qe = qe.max(e);
        }
        if qe <= ddd[qi][qj] {
            continue;
        }
        ddd[qi][qj] = qe;
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let (ni, nj) = (qi as i32 + di, qj as i32 + dj);
            if ni < 0 || h as i32 <= ni || nj < 0 || w as i32 <= nj {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if ddd[ni][nj] < qe && aaa[ni][nj] != '#' {
                bh.push((qe - 1, (ni, nj)));
            }
        }
    }
    println!("No");
}
