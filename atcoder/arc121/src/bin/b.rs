#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aacc: [(isize, char); 2 * n]
    };
    if n == 1 {
        let rs = if aacc[0].1 == aacc[1].1 {
            0
        } else {
            (aacc[0].0 - aacc[1].0).abs()
        };
        println!("{}", rs);
        return;
    }
    let mut vv = vec![vec![]; 3];
    for (a, c) in aacc {
        let target = match c {
            'R' => &mut vv[0],
            'G' => &mut vv[1],
            'B' => &mut vv[2],
            _ => unreachable!(),
        };
        target.push(a);
    }
    if vv.iter().all(|v| v.len().is_even()) {
        println!("0");
        return;
    }
    vv.sort_unstable_by_key(|v| v.len().is_odd());
    for v in vv.iter_mut() {
        v.sort_unstable();
        v.dedup();
    }
    let two_min = get_min_distance(&vv[1], &vv[2]);
    let four_min = get_min_distance(&vv[0], &vv[1]) + get_min_distance(&vv[0], &vv[2]);
    println!("{}", two_min.min(four_min));
}

// ソート済みを2つ渡す
fn get_min_distance(mae: &[isize], ato: &[isize]) -> isize {
    let mut l = 0;
    let mut rs = std::isize::MAX;
    for &m in mae.iter() {
        match ato[l..].binary_search(&m) {
            Ok(_) => {
                rs = 0;
                break;
            }
            Err(i) => {
                if l + i >= ato.len() {
                    rs = rs.min(m - (ato.last().unwrap()).abs());
                    break;
                }
                let indexes = if l + i > 0 {
                    vec![l + i - 1, l + i]
                } else {
                    vec![l + i]
                };
                let min = indexes
                    .into_iter()
                    .map(|j| (*ato.get(j).unwrap_or(&std::isize::MAX) - m).abs())
                    .min()
                    .unwrap();
                l = (l + i).checked_sub(1).unwrap_or_default();
                rs = rs.min(min);
            }
        };
    }
    rs
}
