#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: isize,
        q: isize,
        aa: [Usize1; q]
    };
    let mut kk = vec![k - q; n];
    for a in aa {
        kk[a] += 1;
    }
    for k in kk {
        println!("{}", ["No", "Yes"][(k > 0) as usize]);
    }
}
