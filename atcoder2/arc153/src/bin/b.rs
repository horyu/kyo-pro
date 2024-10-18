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

/*
  123456
2:216543
3:612345

  123456
5:543216
1:561234

X→Yを組み合わせると rorate_right(Y-X) になる
*/

fn main() {
    input! {
        h: usize,
        w: usize,
        mut aaa: [Chars; h],
        q: usize,
        mut aabb: [(isize, isize); q],
    };
    let mut rot_x = 0;
    let mut rot_y = 0;
    for ((a0, b0), (a1, b1)) in aabb.iter().copied().tuples() {
        rot_x += a1 - a0;
        rot_y += b1 - b0;
    }
    rot_x %= h as isize;
    rot_y %= w as isize;
    if rot_x < 0 {
        rot_x += h as isize;
    }
    if rot_y < 0 {
        rot_y += w as isize;
    }
    aaa.rotate_right(rot_x as usize);
    for aa in aaa.iter_mut() {
        aa.rotate_right(rot_y as usize);
    }
    if q % 2 == 1 {
        let (a, b) = aabb[q - 1];
        let (a, b) = (a as usize, b as usize);
        let (u, d) = aaa.split_at_mut(a);
        u.reverse();
        d.reverse();
        for aa in aaa.iter_mut() {
            let (l, r) = aa.split_at_mut(b);
            l.reverse();
            r.reverse();
        }
    }
    for aa in aaa {
        let rs = aa.iter().collect::<String>();
        println!("{rs}");
    }
}
