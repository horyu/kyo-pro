#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        ab: [usize; 2]
    };
    use std::collections::HashSet;
    let abc: HashSet<_> = vec![1, 2, 3].iter().cloned().collect();
    let ab: HashSet<_> = ab.into_iter().collect();
    for x in abc.difference(&ab) {
        println!("{}", x);
    }
}
