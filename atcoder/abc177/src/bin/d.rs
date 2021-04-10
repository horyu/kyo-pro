#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    if m == 0 {
        println!("1");
        return;
    }
    input! {
        aabb: [(Usize1, Usize1); m],
    }
    let mut hss: Vec<HashSet<usize>> = vec![];
    let mut i_to_hss_index: Vec<Option<usize>> = vec![None; n];
    for (a, b) in aabb {
        let hs_index = match (i_to_hss_index[a], i_to_hss_index[b]) {
            (Some(a), Some(b)) if a != b => {
                // 1-4; 2-3; 3-4; のように後から合流する場合に結合できるようにする
                let mut tmp = HashSet::new();
                let (big_index, small) = if hss[a].len() > hss[b].len() {
                    (a, &mut hss[b])
                } else {
                    (b, &mut hss[a])
                };
                std::mem::swap(small, &mut tmp);
                for &j in tmp.iter() {
                    i_to_hss_index[j] = Some(big_index);
                }
                hss[big_index].extend(tmp);
                big_index
            }
            (Some(hs_index), _) | (_, Some(hs_index)) => hs_index,
            (None, None) => {
                let hs_index = hss.len();
                let hs = HashSet::new();
                hss.push(hs);
                hs_index
            }
        };
        let hs = &mut hss[hs_index];
        hs.insert(a);
        hs.insert(b);
        i_to_hss_index[a] = Some(hs_index);
        i_to_hss_index[b] = Some(hs_index);
    }
    let rs = hss.iter().map(|hs| hs.len()).max().unwrap();
    println!("{}", rs);
}
