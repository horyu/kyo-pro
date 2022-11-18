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
        m: usize,
        ss: [String; n],
        tt: [String; m],
    };
    let hs: HashSet<&str> = tt.iter().map(|t| t.as_str()).collect();
    if n == 1 {
        if hs.contains(ss[0].as_str()) || ss[0].len() < 3 {
            println!("-1");
        } else {
            println!("{}", ss[0]);
        }
        return;
    }
    // 最小で結合して超過するか
    let min_len = ss.iter().fold(0, |acc, s| acc + s.len()) + n - 1;
    let over = 16 - min_len;
    // dbg!(over, min_len);
    for ii in (0..n).permutations(n) {
        for over in 0..=over {
            if let Some(s) = dfs(n, &ss, &hs, over, &ii, &mut vec![1; n - 1]) {
                println!("{s}");
                return;
            }
        }
    }
    println!("-1");
}

const UNDERS: [&str; 17] = [
    "",
    "_",
    "__",
    "___",
    "____",
    "_____",
    "______",
    "_______",
    "________",
    "_________",
    "__________",
    "___________",
    "____________",
    "_____________",
    "______________",
    "_______________",
    "________________",
];

fn dfs(
    n: usize,
    ss: &[String],
    hs: &HashSet<&str>,
    over: usize,
    ii: &[usize],
    vv: &mut Vec<usize>,
) -> Option<String> {
    if over == 0 {
        let mut s = String::new();
        for i in 0..(n - 1) {
            s.push_str(ss[ii[i]].as_str());
            s.push_str(UNDERS[vv[i]]);
        }
        s.push_str(ss[ii[n - 1]].as_str());
        if hs.contains(s.as_str()) {
            return None;
        } else {
            return Some(s);
        }
    }
    for i in 0..(n - 1) {
        vv[i] += 1;
        if let Some(s) = dfs(n, ss, hs, over - 1, ii, vv) {
            return Some(s);
        }
        vv[i] -= 1;
    }

    None
}
