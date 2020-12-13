#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut aa: [usize; m]
    };
    if m == n {
        println!("0");
        return;
    }
    if m == 0 {
        println!("1");
        return;
    }
    aa.sort();
    aa.insert(0, 0);
    aa.push(n + 1);
    let mut spaces: Vec<_> = aa.windows(2).map(|tmp| tmp[1] - tmp[0] - 1).collect();
    spaces.sort();
    while let Some(&first) = spaces.first() {
        if first == 0 {
            spaces.remove(0);
        } else {
            break;
        }
    }
    let min = spaces[0];
    let rs = spaces.iter().fold(0, |acc, space| {
        acc + space / min + (space % min != 0) as usize
    });
    println!("{}", rs);
}
