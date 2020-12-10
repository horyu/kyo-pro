#![allow(unused_imports)]
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut aa: [usize; m]
    };
    let mut vv = vec![0; n + 1];
    vv[0] = 1;
    vv[1] = 1;
    if let Some(first) = aa.first() {
        if *first == 1 {
            aa.remove(0);
            vv[1] = 0;
        }
    }
    for i in 2..=n {
        if let Some(first) = aa.first() {
            if *first == i {
                aa.remove(0);
                continue;
            }
        }
        vv[i] = (vv[i - 2] + vv[i - 1]) % 1_000_000_007;
    }
    println!("{}", vv[n]);
}
