#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [isize; n],
        xxyy: [(Usize1, Usize1); m]
    };
    let mut ato = vec![vec![]; n];
    for (x, y) in xxyy {
        ato[x].push(y);
    }
    // 町iより後で最も高く売れる価格
    let mut vv = vec![0; n];
    for i in (0..n).rev() {
        vv[i] = vv[i].max(ato[i].iter().map(|&k| vv[k].max(aa[k])).max().unwrap_or(0));
    }
    let rs = aa
        .into_iter()
        .zip(vv.into_iter())
        .filter_map(|(a, v)| if v > 0 { Some(v - a) } else { None })
        .max()
        .unwrap();
    println!("{rs}");
}
