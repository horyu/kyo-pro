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
        n: usize,
        h: usize,
        w: usize,
        mut aabb: [(usize, usize); n],
    };
    // w < h にしておく
    let (w, h) = if w < h { (w, h) } else { (h, w) };
    // a < b にし、長さが w, h 以下のものだけ残す
    aabb = aabb
        .into_iter()
        .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
        .collect();
    aabb.retain(|&(a, b)| a <= w && b <= h);
    // 面積降順にソート
    aabb.sort_unstable_by_key(|&(a, b)| R(a * b));

    for aabb in (1..=n).flat_map(|cnt| aabb.iter().copied().combinations(cnt)) {
        if aabb.iter().fold(0, |acc, (a, b)| acc + a * b) != h * w {
            continue;
        }
        let mut mat = vec![vec![false; w]; h];
        // eprintln!("{aabb:?}");
        if dfs(&mut mat, &aabb, 0) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn dfs(mat: &mut Vec<Vec<bool>>, aabb: &[(usize, usize)], aabb_idx: usize) -> bool {
    let (w, h) = (mat[0].len(), mat.len());
    let (a, b) = aabb[aabb_idx];
    for (a, b) in [(a, b), (b, a)] {
        // 左上から(a, b)を配置できる座標を探して配置
        for i in 0..h {
            for j in 0..w {
                if i + a <= h && j + b <= w && (0..a).all(|x| (0..b).all(|y| !mat[i + x][j + y])) {
                    for x in 0..a {
                        for y in 0..b {
                            mat[i + x][j + y] = true;
                        }
                    }
                    if aabb_idx + 1 == aabb.len() {
                        return true;
                    }
                    if dfs(mat, aabb, aabb_idx + 1) {
                        return true;
                    }
                    for x in 0..a {
                        for y in 0..b {
                            mat[i + x][j + y] = false;
                        }
                    }
                }
            }
        }
    }
    false
}
