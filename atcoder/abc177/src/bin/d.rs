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
        // 1-4; 2-3; 3-4; のように後から合流する場合に結合できるようにする
        if let Some(hs_index) = i_to_hss_index[a].or(i_to_hss_index[b]) {
            let hs = hss.get_mut(hs_index).unwrap();
            hs.insert(a);
            hs.insert(b);
            i_to_hss_index[a] = Some(hs_index);
            i_to_hss_index[b] = Some(hs_index);
        } else {
            let hs_index = hss.len();
            let mut hs = HashSet::new();
            hs.insert(a);
            hs.insert(b);
            hss.push(hs);
            i_to_hss_index[a] = Some(hs_index);
            i_to_hss_index[b] = Some(hs_index);
        }
    }
    let rs = hss.iter().map(|hs| hs.len()).max().unwrap();
    println!("{}", rs);
}
