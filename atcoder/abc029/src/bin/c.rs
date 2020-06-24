#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let abc = vec!['a', 'b', 'c'];
    let mut s = String::new();
    func(&mut s, &abc, n, "");
    print!("{}", s);
}

// 文字を受け取って n>0 なら abc それぞれに対して再帰、 n==0 ならsにpush
fn func(s: &mut String, abc: &Vec<char>, n: usize, tmp: &str) {
    if n == 0 {
        s.push_str(tmp);
        s.push('\n');
    } else {
        for &char in abc {
            let mut tmp = String::from(tmp);
            tmp.push(char);
            func(s, abc, n - 1, &tmp);
        }
    }
}
