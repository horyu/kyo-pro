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
        s: [Chars; 9],
    };
    let s = s
        .into_iter()
        .map(|s| s.into_iter().map(|c| c == '#').collect_vec())
        .collect_vec();
    let mut iijj = vec![];
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] {
                iijj.push((i as isize, j as isize));
            }
        }
    }
    let mut rs = 0usize;
    for vv in iijj.into_iter().combinations(2) {
        if let [(x0, y0), (x1, y1)] = vv[..] {
            let dx = x1 - x0;
            let dy = y1 - y0;
            let x2 = x1 + dy;
            let y2 = y1 - dx;
            let x3 = x0 + dy;
            let y3 = y0 - dx;
            if [x2, y2, x3, y3].into_iter().all(|w| (0..9).contains(&w))
                && s[x2 as usize][y2 as usize]
                && s[x3 as usize][y3 as usize]
            {
                rs += 1;
            }
        }
    }
    rs /= 2; // 逆順も集計するので半分にする
    println!("{rs}");
}
