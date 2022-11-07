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
        t: usize,
        sss: [Chars; h],
    };
    let (mut si, mut sj, mut gi, mut gj) = (0, 0, 0, 0);
    for (i, ss) in sss.iter().enumerate() {
        for (j, &s) in ss.iter().enumerate() {
            match s {
                'S' => {
                    si = i;
                    sj = j;
                }
                'G' => {
                    gi = i;
                    gj = j;
                }
                _ => (),
            }
        }
    }
    let mut vvv = vec![vec![BTreeMap::new(); w]; h];
    vvv[si][sj].insert(0usize, 0usize);
    let mut qq = VecDeque::new();
    qq.push_back((si, sj, 0, 0));
    // 9 + 9 + 9 + 7 + 7 + 5 + 5 + 3 + 3 + 1 + 1
    for _ in 0..100 {
        let mut new_qq = VecDeque::new();
        while let Some((qi, qj, qw, qb)) = qq.pop_front() {
            if qi.abs_diff(gi) + qj.abs_diff(gj) == 1 {
                if let Some(e) = vvv[gi][gj].get_mut(&qb) {
                    *e = (*e).min(qw + 1);
                } else {
                    vvv[gi][gj].insert(qb, qw + 1);
                }
                continue;
            }
            let mut rr = vec![];
            if 0 < qi {
                rr.push((qi - 1, qj));
            }
            if qi < h - 1 {
                rr.push((qi + 1, qj));
            }
            if 0 < qj {
                rr.push((qi, qj - 1));
            }
            if qj < w - 1 {
                rr.push((qi, qj + 1));
            }
            for (qii, qjj) in rr {
                let x = (sss[qii][qjj] == '#') as usize;
                let qbb = qb + x;
                let qww = qw + 1 - x;
                if vvv[qii][qjj]
                    .range(0..=qbb)
                    .any(|(&bb, &ww)| bb <= qbb && ww <= qww)
                {
                    continue;
                }
                if let Some(e) = vvv[qii][qjj].get_mut(&qbb) {
                    if qww < *e {
                        *e = qww;
                        new_qq.push_back((qi, qjj, qww, qbb));
                    }
                } else {
                    vvv[qii][qjj].insert(qbb, qww);
                    new_qq.push_back((qii, qjj, qww, qbb));
                }
            }
        }
        qq = new_qq;
    }
    // dbg!(&vvv[gi][gj]);
    let rs = vvv[gi][gj]
        .iter()
        .map(|(&bb, &ww)| t.saturating_sub(ww) / bb)
        .max()
        .unwrap();
    println!("{rs}");
}
