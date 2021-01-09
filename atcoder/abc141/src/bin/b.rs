#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let tf = s.iter().step_by(2).all(|c| matches!(c, 'R' | 'U' | 'D'))
        && s.iter()
            .skip(1)
            .step_by(2)
            .all(|c| matches!(c, 'L' | 'U' | 'D'));
    println!("{}", ["No", "Yes"][tf as usize]);
}
