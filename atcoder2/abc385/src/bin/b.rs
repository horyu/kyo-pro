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
        mut x: Usize1,
        mut y: Usize1,
        ss: [Chars; h],
        t: Chars,
    };
    let mut hs = HashSet::new();
    for tc in t {
        match tc {
            'U' => {
                if 0 < x && ss[x - 1][y] != '#' {
                    x -= 1;
                }
            }
            'D' => {
                if x + 1 < h && ss[x + 1][y] != '#' {
                    x += 1;
                }
            }
            'L' => {
                if 0 < y && ss[x][y - 1] != '#' {
                    y -= 1;
                }
            }
            'R' => {
                if y + 1 < w && ss[x][y + 1] != '#' {
                    y += 1;
                }
            }
            _ => unreachable!(),
        }
        if ss[x][y] == '@' {
            hs.insert((x, y));
        }
    }
    println!("{} {} {}", x + 1, y + 1, hs.len());
}
