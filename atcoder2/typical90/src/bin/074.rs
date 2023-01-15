#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let ii = s
        .iter()
        .copied()
        .map(|c| match c {
            'a' => 0,
            'b' => 1,
            _ => 2,
        })
        .collect_vec();

    let mut hm = HashMap::new();
    for i in 0..3 {
        hm.insert(vec![i], i);
    }

    let rs = dfs(&ii, &mut hm);
    println!("{rs}");
}

fn dfs(ii: &[usize], hm: &mut HashMap<Vec<usize>, usize>) -> usize {
    if let Some(cnt) = hm.get(&ii.to_vec()).copied() {
        return cnt;
    }

    let len = ii.len();
    let init_cnt = dfs(&ii[..(len - 1)], hm);

    let rs = match ii[len - 1] {
        0 => init_cnt,
        1 => init_cnt + dfs(&vec![1; len - 1], hm) + 1,
        _ => init_cnt + dfs(&vec![1; len - 1], hm) * 2 + 2,
    };
    hm.insert(ii.to_vec(), rs);

    rs
}
