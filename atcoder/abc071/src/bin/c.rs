#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.sort_unstable();
    aa.reverse();

    let mut pre_a = 0;
    let mut count = 0;
    let mut pairs = vec![];
    for a in aa {
        if a == pre_a {
            count += 1;
            if count == 2 {
                pairs.push(a);
                if pairs.len() == 2 {
                    println!("{}", pairs[0] * pairs[1]);
                    return;
                }
                count = 0;
            }
        } else {
            pre_a = a;
            count = 1;
        }
    }
    println!("0");
}
