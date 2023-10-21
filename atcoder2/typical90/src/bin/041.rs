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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn cross(p: Point, q: Point) -> isize {
    p.x * q.y - p.y * q.x
}

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n],
    };
    // https://github.com/E869120/kyopro_educational_90/blob/main/sol/041-03.cpp
    let pp = xxyy
        .iter()
        .map(|&(x, y)| Point { x, y })
        .sorted_unstable()
        .collect_vec();
    let mut g1 = vec![pp[0], pp[1]];
    let mut g2 = g1.clone();
    for i in 2..n {
        loop {
            let len = g1.len();
            if 2 <= len && cross(g1[len - 1] - g1[len - 2], pp[i] - g1[len - 1]) <= 0 {
                g1.pop();
            } else {
                break;
            }
        }
        loop {
            let len = g2.len();
            if 2 <= len && cross(g2[len - 1] - g2[len - 2], pp[i] - g2[len - 1]) >= 0 {
                g2.pop();
            } else {
                break;
            }
        }
        g1.push(pp[i]);
        g2.push(pp[i]);
    }
    let mut totsuhou = g1;
    totsuhou.extend(g2[1..(g2.len() - 1)].iter().rev());

    let len = totsuhou.len();
    let mut edge_point = len;
    for i in 0..len {
        let Point { x: ax, y: ay } = totsuhou[i];
        let Point { x: bx, y: by } = totsuhou[(i + 1) % len];
        let vx = ax.abs_diff(bx);
        let vy = ay.abs_diff(by);
        let r = vx.gcd(&vy);
        edge_point += r - 1;
    }

    let mut area = 0;
    for i in 0..len {
        let Point { x: ax, y: ay } = totsuhou[i];
        let Point { x: bx, y: by } = totsuhou[(i + 1) % len];
        area += (bx - ax) * (by + ay);
    }
    let area = area.unsigned_abs();
    let rs = (area + edge_point + 2) / 2 - n;
    println!("{rs}");
}
