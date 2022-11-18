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
        aa: [usize; n],
    };
    let mut rs = 0;
    let mut vv = vec![false; 1e6 as usize + 1];
    for (a, cnt) in aa
        .into_iter()
        .counts()
        .into_iter()
        .sorted_unstable_by_key(|kv| kv.0)
    {
        if 1 == cnt {
            let mut ok = true;
            for i in (1usize..).take_while(|i| i * i <= a) {
                if a % i == 0 && (vv[i] || vv[a / i]) {
                    ok = false;
                    break;
                }
            }
            if ok {
                rs += 1;
            }
        }
        vv[a] = true;
    }
    println!("{rs}");
}
