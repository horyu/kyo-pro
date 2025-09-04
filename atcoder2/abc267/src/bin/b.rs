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

fn main() {
    input! {
        s: Chars,
    };
    if s[0] == '0' {
        let vvv = vec![
            vec![6],
            vec![3],
            vec![1, 7],
            vec![0, 4],
            vec![2, 8],
            vec![5],
            vec![9],
        ];
        for (i, j) in (0..vvv.len()).tuple_combinations() {
            if vvv[i].iter().any(|&v| s[v] == '1')
                && vvv[j].iter().any(|&v| s[v] == '1')
                && ((i + 1)..j).any(|k| vvv[k].iter().all(|&v| s[v] == '0'))
            {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
