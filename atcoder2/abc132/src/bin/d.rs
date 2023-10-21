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
        k: usize,
    };
    // https://drken1215.hatenablog.com/entry/2018/06/08/210000
    let mut com = vec![vec![0usize; 2001]; 2001];
    com[0][0] = 1;
    for i in 1..2001 {
        com[i][0] = 1;
        for j in 1..2001 {
            com[i][j] = (com[i - 1][j - 1] + com[i - 1][j]) % 1000000007;
        }
    }
    for t in 1..=k {
        // n-k個の赤玉の左右を含めた隙間のt個に青玉を配置 n-k+1 C t
        // 隙間t個に青玉k個を配置 = k-1個の間にt-1個仕切りを入れる k-1 C t-1
        let rs = com[n - k + 1][t] * com[k - 1][t - 1] % 1000000007;
        println!("{rs}");
    }
}
