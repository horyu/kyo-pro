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
    let ngs: HashSet<String> = tt.into_iter().collect();
    let remain = 16 - (n - 1) - ss.iter().fold(0, |acc, s| acc + s.len());
    for strs in ss.iter().map(|s| s.as_str()).permutations(n) {
        if let Some(rs) = dfs(0, remain as isize, &strs, &ngs, &mut String::new()) {
            println!("{rs}");
            return;
        }
    }
    println!("-1");
}

fn dfs(
    cur: usize,
    remain: isize,
    strs: &[&str],
    ngs: &HashSet<String>,
    ans: &mut String,
) -> Option<String> {
    if remain < 0 {
        return None;
    }
    if cur == strs.len() {
        if 3 <= ans.len() && !ngs.contains(ans) {
            return Some(ans.to_owned());
        }
        return None;
    }

    if !ans.is_empty() && !ans.ends_with('_') {
        ans.push('_');
        dfs(cur, remain, strs, ngs, ans)
    } else {
        let mut new_ans = ans.clone();
        new_ans.push_str(strs[cur]);
        match dfs(cur + 1, remain, strs, ngs, &mut new_ans) {
            None => {
                if !ans.is_empty() {
                    ans.push('_');
                    dfs(cur, remain - 1, strs, ngs, ans)
                } else {
                    None
                }
            }
            rs => rs,
        }
    }
}
