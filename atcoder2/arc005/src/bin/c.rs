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
        ccc: [Chars; h],
    };
    let hh = h as isize - 1;
    let ww = w as isize - 1;
    let (mut si, mut sj, mut gi, mut gj) = (0, 0, 0, 0);
    for (i, cc) in ccc.iter().enumerate() {
        for (j, c) in cc.iter().copied().enumerate() {
            match c {
                's' => (si, sj) = (i, j),
                'g' => (gi, gj) = (i, j),
                _ => {}
            }
        }
    }

    let mut qq = VecDeque::new();
    let mut dd = vec![vec![3; w]; h];
    qq.push_front((si, sj, 0));
    dd[si][sj] = 0;

    while let Some((qi, qj, qd)) = qq.pop_front() {
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().copied() {
            let (ni, nj) = (qi as isize + di, qj as isize + dj);
            if ni < 0 || hh < ni || nj < 0 || ww < nj {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            let nd = qd + usize::from(ccc[ni][nj] == '#');
            if nd < dd[ni][nj] {
                if (ni, nj) == (gi, gj) {
                    println!("YES");
                    return;
                }

                dd[ni][nj] = nd;
                if nd == qd {
                    qq.push_front((ni, nj, nd));
                } else {
                    qq.push_back((ni, nj, nd));
                }
            }
        }
    }

    println!("NO");
}
