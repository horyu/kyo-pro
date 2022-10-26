#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        aaa: [Chars; 10],
    };
    let mut checked = [[false; 10]; 10];
    let mut hhss = vec![];
    for i in 0..10 {
        for j in 0..10 {
            if aaa[i][j] == 'o' && !checked[i][j] {
                checked[i][j] = true;
                let mut hs = HashSet::new();
                let mut qq = VecDeque::new();
                qq.push_back((i, j));
                while let Some((qi, qj)) = qq.pop_front() {
                    let mut ii = vec![];
                    let mut jj = vec![];
                    if 0 < qi {
                        ii.push(qi - 1);
                    }
                    if qi < 9 {
                        ii.push(qi + 1);
                    }
                    if 0 < qj {
                        jj.push(qj - 1);
                    }
                    if qj < 9 {
                        jj.push(qj + 1);
                    }
                    for (pi, pj) in iproduct!([qi], jj).chain(iproduct!(ii, [qj])) {
                        if checked[pi][pj] {
                            continue;
                        }
                        if aaa[pi][pj] == 'o' {
                            qq.push_back((pi, pj));
                            checked[pi][pj] = true;
                        } else {
                            hs.insert((pi, pj));
                        }
                    }
                }
                hhss.push(hs);
            }
        }
    }
    let len = hhss.len();
    let mut hm = HashMap::new();
    for hs in hhss {
        for k in hs {
            *hm.entry(k).or_insert(0) += 1;
        }
    }
    let tf = hm.into_values().contains(&len);
    let rs = if tf { "YES" } else { "NO" };
    println!("{rs}");
}
