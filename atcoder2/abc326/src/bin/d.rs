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
use std::vec;

fn main() {
    input! {
        n: usize,
        rr: Chars,
        cc: Chars,
    };
    // Python: for a,b,c in itertools.product(itertools.permutations(range(N)),repeat=3):
    for www in (0..3)
        .map(|_| (0..n).permutations(n))
        .multi_cartesian_product()
    {
        if izip!(&www[0], &www[1], &www[2]).any(|(a, b, c)| a == b || b == c || c == a) {
            continue;
        }
        let mut mat = vec![vec!['.'; n]; n];
        for (cdiff, ww) in www.iter().enumerate() {
            for (i, j) in ww.iter().copied().enumerate() {
                mat[i][j] = (b'A' + cdiff as u8) as char;
            }
        }
        if rr
            .iter()
            .copied()
            .enumerate()
            .all(|(i, r)| mat[i].iter().find(|&&c| c != '.').copied().unwrap_or('.') == r)
            && cc
                .iter()
                .copied()
                .enumerate()
                .all(|(j, c)| (0..n).map(|i| mat[i][j]).find(|&c| c != '.').unwrap_or('.') == c)
        {
            println!("Yes");
            for i in 0..n {
                println!("{}", mat[i].iter().join(""));
            }
            return;
        }
    }
    println!("No");
}
