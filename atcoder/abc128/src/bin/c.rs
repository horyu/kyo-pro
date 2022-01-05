#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut ssss = vec![];
    for _ in 0..m {
        input! {k: usize};
        input! {ss: [Usize1; k]};
        ssss.push(ss);
    }
    input! {pp: [usize; m]};

    let rs = (0..n)
        .map(|_| [false, true])
        .multi_cartesian_product()
        .filter(|ttff| {
            ssss.iter()
                .zip(pp.iter())
                .all(|(ss, p)| ss.iter().filter(|s| ttff[**s]).count() % 2 == *p)
        })
        .count();
    println!("{}", rs);
}
