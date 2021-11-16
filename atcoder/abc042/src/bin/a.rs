#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut abc: [u8; 3]
    };
    abc.sort_unstable();
    if (abc[0] == abc[1]) && (abc[0] == 5) && (abc[2] == 7) {
        println!("YES");
    } else {
        println!("NO");
    }
}
