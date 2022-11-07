#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        ccc: [Chars; h]
    };
    let mut si = 0;
    let mut sj = 0;
    let mut vvv = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            match ccc[i][j] {
                'S' => {
                    si = i;
                    sj = j;
                }
                '.' => {
                    vvv[i][j] = true;
                }
                _ => (),
            }
        }
    }
    // dbg!(si, sj);
    let check = |ssi: usize, ssj: usize| -> bool {
        if !vvv[ssi][ssj] {
            return false;
        }
        let mut pushed = vec![vec![false; w]; h];
        pushed[ssi][ssj] = true;
        let mut qq = VecDeque::new();
        qq.push_back((ssi, ssj));
        let mut is_first = true;
        while let Some((qi, qj)) = qq.pop_front() {
            if is_first {
                is_first = false;
            } else if (qi.abs_diff(si) + qj.abs_diff(sj)) == 1 {
                // dbg!(qi, qj);
                return true;
            }
            let mut xxyy = vec![];
            if 0 < qi {
                xxyy.push((qi - 1, qj));
            }
            if qi < h - 1 {
                xxyy.push((qi + 1, qj));
            }
            if 0 < qj {
                xxyy.push((qi, qj - 1));
            }
            if qj < w - 1 {
                xxyy.push((qi, qj + 1));
            }
            for (x, y) in xxyy {
                if vvv[x][y] && !pushed[x][y] {
                    qq.push_back((x, y));
                    pushed[x][y] = true;
                }
            }
        }
        false
    };
    let mut tf = false;
    if !tf && 0 < si {
        tf = check(si - 1, sj);
    }
    if !tf && si < h - 1 {
        tf = check(si + 1, sj);
    }
    if !tf && 0 < sj {
        tf = check(si, sj - 1);
    }
    if !tf && sj < w - 1 {
        tf = check(si, sj + 1);
    }
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
