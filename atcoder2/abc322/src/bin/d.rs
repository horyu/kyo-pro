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
    let iter = pppp
        .into_iter()
        .map(|ppp| {
            let mut tmp = [[false; 4]; 4];
            for (i, pp) in ppp.into_iter().enumerate() {
                for (j, p) in pp.into_iter().enumerate() {
                    tmp[i][j] = p == '#';
                }
            }
            tmp
        })
        .map(|ppp| {
            let mut rotates = vec![ppp];
            for k in 0..3 {
                let mut rotated = [[false; 4]; 4];
                for (i, pp) in ppp.iter().enumerate() {
                    for (j, p) in pp.iter().enumerate() {
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
        'd0: for (dx0, dy0) in dxdy.iter() {
            let mut base0 = [[false; 12]; 12];
            let mut cnt0 = 0;
            for (i, qq) in qqq[0].iter().enumerate() {
                for (j, &tf) in qq.iter().enumerate() {
                    if tf {
                        let ii = i + dx0;
                        let jj = j + dy0;
                        if (4..8).contains(&ii) && (4..8).contains(&jj) {
                            base0[ii][jj] = true;
                            cnt0 += 1;
                        } else {
                            continue 'd0;
                        }
                    }
                }
            }
            'd1: for (dx1, dy1) in dxdy.iter() {
                let mut base1 = base0;
                let mut cnt1 = cnt0;
                for (i, qq) in qqq[1].iter().enumerate() {
                    for (j, &tf) in qq.iter().enumerate() {
                        if tf {
                            let ii = i + dx1;
                            let jj = j + dy1;
                            if !base1[ii][jj] && (4..8).contains(&ii) && (4..8).contains(&jj) {
                                base1[ii][jj] = true;
                                cnt1 += 1;
                            } else {
                                continue 'd1;
                            }
                        }
                    }
                }
                'd2: for (dx2, dy2) in dxdy.iter() {
                    let mut base2 = base1;
                    let mut cnt2 = cnt1;
                    for (i, qq) in qqq[2].iter().enumerate() {
                        for (j, &tf) in qq.iter().enumerate() {
                            if tf {
                                let ii = i + dx2;
                                let jj = j + dy2;
                                if !base2[ii][jj] && (4..8).contains(&ii) && (4..8).contains(&jj) {
                                    base2[ii][jj] = true;
                                    cnt2 += 1;
                                } else {
                                    continue 'd2;
                                }
                            }
                        }
                    }
                    if cnt2 == 16 {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
