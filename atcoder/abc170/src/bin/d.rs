#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.sort();
    let mut count = 0;
    let mut i = 0;
    loop {
        if i >= aa.len() {
            break;
        }
        let ai = aa[i];
        let mut j = i + 1;
        loop {
            if j >= aa.len() {
                break;
            }
            let aj = aa[j];
            if aj == ai {
                count += 1;
                while (j < aa.len()) && (aa[j] == ai) {
                    aa.remove(j);
                }
                continue;
            }
            if aj % ai == 0 {
                aa.remove(j);
                continue;
            }
            j += 1;
        }
        i += 1;
    }
    println!("{}", aa.len() - count);
}
