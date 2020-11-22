#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut abc: [usize; 3],
    };
    abc.retain(|&x| x != 0);
    let i_min = 100 - abc.iter().max().unwrap();
    let i_max = abc.iter().fold(1, |acc, x| acc + 99 - x);
    let mut sum = 0.0;
    for i in i_min..=i_max {
        let reachables: Vec<_> = abc.iter().filter(|&x| i + x >= 100).collect();
        // let unreachables: Vec<_> = abc.iter().filter(|&x| i + x < 100).collect();
        for reachable in reachables {
            let yoyuu = i - reachable;
            // reachableを99枚、yoyuu回ソレ以外を99枚以下になるようにとる組み合わせ

        }

    }
    // println!("{}", );
}
