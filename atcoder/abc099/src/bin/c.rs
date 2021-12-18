#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize // <= 100000 = 1e5
    };
    // (->(a){ (1..).take_while{|i| a ** i <= 100000}.map{|i| a**i }}).call(9)
    // let nine = vec![9usize, 81, 729, 6561, 59049];
    // let six = vec![6usize, 36, 216, 1296, 7776, 46656];
    let kouho = vec![
        1usize, 6, 9, 36, 81, 216, 729, 1296, 6561, 7776, 46656, 59049,
    ];
    let mut v = vec![0usize; 100001];
    for i in 1..=n {
        v[i] = kouho
            .iter()
            .filter_map(|&j| i.checked_sub(j).map(|pre| v[pre] + 1))
            .min()
            .unwrap();
    }
    println!("{}", v[n]);
}
