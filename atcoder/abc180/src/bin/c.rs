#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    // https://qiita.com/drken/items/a14e9af0ca2d857dad23#3-%E7%B4%84%E6%95%B0%E5%88%97%E6%8C%99
    let mut mae = vec![];
    let mut ato = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            mae.push(i);
            if n / i != i {
                ato.push(n / i);
            }
        }
        i += 1;
    }
    for x in mae.iter().chain(ato.iter().rev()) {
        println!("{}", x);
    }
}
