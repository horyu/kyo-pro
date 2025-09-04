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
        ha: isize,
        wa: isize,
        aa: [Chars; ha],
        hb: isize,
        wb: isize,
        bb: [Chars; hb],
        hx: isize,
        wx: isize,
        xx: [Chars; hx],
    };
    fn to_iijj(vvv: &[Vec<char>]) -> Vec<(isize, isize)> {
        let mut rs = vec![];
        for (i, vv) in vvv.iter().enumerate() {
            for (j, &v) in vv.iter().enumerate() {
                if v == '#' {
                    rs.push((i as isize, j as isize));
                }
            }
        }
        rs
    }
    let wwa = to_iijj(&aa);
    let wwb = to_iijj(&bb);
    let wwx = to_iijj(&xx);

    let base: HashSet<(isize, isize)> = wwa.iter().copied().collect();
    for x in -20..=20 {
        for y in -20..=20 {
            let mut hs = base.clone();
            for (i, j) in wwb.iter().copied() {
                hs.insert((i + x, j + y));
            }
            if hs.len() == wwx.len()
                && izip!(hs.into_iter().sorted_unstable(), wwx.iter().copied())
                    .map(|(p, q)| (p.0 - q.0, p.1 - q.1))
                    .all_equal()
            {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
