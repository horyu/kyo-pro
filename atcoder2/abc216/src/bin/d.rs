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
        n: usize,
        m: usize,
    };
    let mut aaa = vec![];
    let mut atoii: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut pair = vec![];
    for i in 0..m {
        input! {mut k: usize, aa: [usize; k]};
        let a = aa[k - 1];
        if let Some(ii) = atoii.get_mut(&a) {
            ii.push(i);
            pair.push(a);
        } else {
            atoii.insert(a, vec![i]);
        }
        aaa.push(aa);
    }
    let mut cnt = 0;
    while let Some(a) = pair.pop() {
        cnt += 1;
        for i in atoii.remove(&a).unwrap() {
            aaa[i].pop();
            if let Some(&new_a) = aaa[i].last() {
                if let Some(ii) = atoii.get_mut(&new_a) {
                    ii.push(i);
                    pair.push(new_a);
                } else {
                    atoii.insert(new_a, vec![i]);
                }
            }
        }
    }
    let rs = ["No", "Yes"][(cnt == n) as usize];
    println!("{rs}");
}
