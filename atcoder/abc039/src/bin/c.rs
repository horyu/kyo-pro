#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String
    };
    // WBWBWWBWBWBW
    let oto = ["Do", "", "Re", "", "Mi", "Fa", "", "So", "", "La", "", "Si"];
    let kenban3 = "WBWBWWBWBWBWWBWBWWBWBWBWWBWBWWBWBWBW";
    for left in 0.. {
        if kenban3[left..(left + s.len())] == s {
            let index = left % oto.len();
            println!("{}", oto[index]);
            return;
        }
    }
}
