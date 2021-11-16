#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        dd: [usize; k]
    };
    let usable_head = (1usize..=9).filter(|i| !dd.contains(i)).collect_vec();
    if n < 10 {
        if let Some(i) = usable_head.iter().find(|&&i| i >= n) {
            println!("{}", i);
            return;
        }
    }
    let order = (1..std::u32::MAX)
        .find(|&o| n / 10usize.pow(o) == 0)
        .unwrap();
    let usable_body = (0usize..=9).filter(|i| !dd.contains(i)).collect_vec();
    for i in order..std::u32::MAX {
        for &head in &usable_head {
            for vv in (0..(i - 1))
                .map(|_| usable_body.iter())
                .multi_cartesian_product()
            {
                let body = vv.into_iter().fold(0usize, |acc, v| acc * 10 + v);
                let num = body + head * 10usize.pow(i - 1);
                if num % n == 0 {
                    println!("{}", num);
                    return;
                }
            }
        }
    }
}
