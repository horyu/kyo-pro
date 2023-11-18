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
        h: usize,
        w: usize,
        sss: [Chars; h],
    };
    let sss = sss
        .into_iter()
        .map(|ss| ss.into_iter().map(|c| c == '#').chain([true]).collect_vec())
        .chain([vec![true; w + 1]])
        .collect_vec();
    fn judge(sss: &Vec<Vec<bool>>, i: usize, j: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if sss[i][j] {
            return true;
        }
        if let Some(tf) = memo[i][j] {
            return tf;
        }
        let tf = !judge(sss, i + 1, j, memo)
            || !judge(sss, i, j + 1, memo)
            || !judge(sss, i + 1, j + 1, memo);
        memo[i][j] = Some(tf);
        tf
    }
    if judge(&sss, 0, 0, &mut vec![vec![None; w + 1]; h + 1]) {
        println!("First");
    } else {
        println!("Second");
    }
}
