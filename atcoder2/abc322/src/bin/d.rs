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
        pppp: [[Chars; 4]; 3]
    };
    // [[[bool; 4]; 4]; 3] に変換する
    let mut qqqq = [[[false; 4]; 4]; 3];
    let mut cnt = 0;
    for (k, ppp) in pppp.iter().enumerate() {
        for (i, pp) in ppp.iter().enumerate() {
            for (j, &p) in pp.iter().enumerate() {
                if p == '#' {
                    qqqq[k][i][j] = true;
                    cnt += 1;
                }
            }
        }
    }
    if cnt != 16 {
        println!("No");
        return;
    }
    let iter = qqqq
        .into_iter()
        .enumerate()
        .map(|(qi, qqq)| {
            let mut rotates = vec![qqq];
            if qi == 0 {
                return rotates;
            }
            for k in 0..3 {
                let mut rotated = [[false; 4]; 4];
                for (i, qq) in qqq.iter().enumerate() {
                    for (j, q) in qq.iter().enumerate() {
                        rotated[i][j] = rotates[k][3 - j][i];
                    }
                }
                rotates.push(rotated);
            }
            // for r in rotates.iter() {
            //     for rr in r.iter() {
            //         eprintln!(
            //             "{}",
            //             rr.iter().map(|&tf| if tf { "#" } else { "." }).join("")
            //         );
            //     }
            // }
            rotates
        })
        .multi_cartesian_product();
    let dxdy = iproduct!(1usize..=7, 1usize..=7).collect_vec();
    for qqq in iter {
        // 3つのパターンを平行移動だけして4x4の範囲が全てtrueになるかを確認する
        for (dx0, dy0) in dxdy.iter() {
            let mut base0 = [[false; 12]; 12];
            for (i, qq) in qqq[0].iter().enumerate() {
                for (j, &tf) in qq.iter().enumerate() {
                    if tf {
                        base0[i + dx0][j + dy0] = true;
                    }
                }
            }
            for (dx1, dy1) in dxdy.iter() {
                let mut base1 = base0;
                for (i, qq) in qqq[1].iter().enumerate() {
                    for (j, &tf) in qq.iter().enumerate() {
                        if tf {
                            base1[i + dx1][j + dy1] = true;
                        }
                    }
                }
                for (dx2, dy2) in dxdy.iter() {
                    let mut base2 = base1;
                    for (i, qq) in qqq[2].iter().enumerate() {
                        for (j, &tf) in qq.iter().enumerate() {
                            if tf {
                                base2[i + dx2][j + dy2] = true;
                            }
                        }
                    }
                    if (4..8).all(|i| (4..8).all(|j| base2[i][j])) {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
