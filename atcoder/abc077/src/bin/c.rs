#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
        mut bb: [usize; n],
        mut cc: [usize; n],
    };
    aa.sort_unstable();
    bb.sort_unstable();
    cc.sort_unstable();
    let mut a_i = 0;
    let mut c_i = 0;
    // bを基準に自身未満のaa, 自身以下のcc(nから引けば自身超過)の数を求める
    let mut rs = 0;
    for b in bb {
        while let Some(next_a) = aa.get(a_i) {
            if *next_a < b {
                a_i += 1;
            } else {
                break;
            }
        }
        while let Some(next_c) = cc.get(c_i) {
            if *next_c <= b {
                c_i += 1;
            } else {
                break;
            }
        }
        rs += a_i * (n - c_i);
    }
    println!("{}", rs);
}
