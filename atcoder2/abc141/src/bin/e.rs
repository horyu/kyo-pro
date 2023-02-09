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
    // https://drken1215.hatenablog.com/entry/2019/09/16/014600
    let mut rs = 0;
    for i in 0..n {
        let lcp = zalgo(&s[i..]);
        for j in 0..lcp.len() {
            let l = lcp[j].min(j);
            rs = rs.max(l);
        }
    }
    println!("{rs}");
}

fn zalgo(s: &[char]) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; n];
    res[0] = n;
    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < n && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}
