#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use num_traits::Saturating;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [Chars; n],
    };
    let mut checked = vec![vec![[false; 4]; m]; n];
    let mut qq = VecDeque::new();
    qq.push_back((1, 1));
    let mut hs = HashSet::new();
    while let Some((qx, qy)) = qq.pop_front() {
        hs.insert((qx, qy));
        if !checked[qx][qy][0] {
            checked[qx][qy][0] = true;
            for x in (0..=qx).rev() {
                hs.insert((x, qy));
                if ss[x.saturating_sub(1)][qy] == '#' {
                    if !checked[x][qy][0] {
                        checked[x][qy][0] = true;
                        qq.push_back((x, qy));
                        // eprintln!("0 {:?} -> {:?}", (qx, qy), (x, qy))
                    }
                    break;
                }
            }
        }
        if !checked[qx][qy][1] {
            checked[qx][qy][1] = true;
            for x in qx..n {
                hs.insert((x, qy));
                if ss[(x + 1).min(n - 1)][qy] == '#' {
                    if !checked[x][qy][1] {
                        checked[x][qy][1] = true;
                        qq.push_back((x, qy));
                        // eprintln!("1 {:?} -> {:?}", (qx, qy), (x, qy))
                    }
                    break;
                }
            }
        }
        if !checked[qx][qy][2] {
            checked[qx][qy][2] = true;
            for y in (0..=qy).rev() {
                hs.insert((qx, y));
                if ss[qx][y.saturating_sub(1)] == '#' {
                    if !checked[qx][y][2] {
                        checked[qx][y][2] = true;
                        qq.push_back((qx, y));
                        // eprintln!("2 {:?} -> {:?}", (qx, qy), (qx, y))
                    }
                    break;
                }
            }
        }
        if !checked[qx][qy][3] {
            checked[qx][qy][3] = true;
            for y in qy..m {
                hs.insert((qx, y));
                if ss[qx][(y + 1).min(m - 1)] == '#' {
                    if !checked[qx][y][3] {
                        checked[qx][y][3] = true;
                        qq.push_back((qx, y));
                        // eprintln!("3 {:?} -> {:?}", (qx, qy), (qx, y))
                    }
                    break;
                }
            }
        }
    }
    let rs = hs.len();
    // for i in 0..n {
    //     eprintln!(
    //         "{}",
    //         (0..m)
    //             .map(|j| ['.', 'o'][hs.contains(&(i, j)) as usize])
    //             .join("")
    //     );
    // }
    println!("{rs}");
}
