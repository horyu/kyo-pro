#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
    };
    let mut aa = vec![];
    for _ in 0..n {
        input! {
            a: usize,
            xxyy: [(Usize1, i32); a]
        };
        aa.push(xxyy.into_iter().map(|xy| (xy.0, xy.1 == 1)).collect_vec());
    }
    let rs = (0..n)
        .map(|_| [true, false])
        .multi_cartesian_product()
        .filter(|ttff| {
            for (i, &tf) in ttff.iter().enumerate().filter(|itf| *itf.1) {
                if ttff[i] != tf {
                    return false;
                }
                for &(j, jtf) in &aa[i] {
                    if ttff[j] != jtf {
                        return false;
                    }
                }
            }
            true
        })
        .map(|ttff| ttff.into_iter().map(|tf| tf as usize).sum::<usize>())
        .max()
        .unwrap_or(0);
    println!("{rs}");
}
