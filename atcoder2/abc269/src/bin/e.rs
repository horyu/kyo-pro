#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::Write;

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim_end().parse::<usize>().unwrap();
    buf.clear();

    let mut il = 1;
    let mut ir = n;
    let mut jl = 1;
    let mut jr = n;
    while il != ir {
        let m = (il + ir) / 2;
        stdout
            .write_fmt(format_args!("? {il} {m} 1 {n}\n"))
            .unwrap();
        stdout.flush().unwrap();

        stdin.read_line(&mut buf).unwrap();
        let k = buf.trim_end().parse::<usize>().unwrap();
        buf.clear();

        if k == (m - il + 1) {
            // (m + 1)..=ir に空きがある
            il = m + 1;
        } else {
            // il..=m に空きがある
            ir = m;
        }
        // eprintln!("il:{il} ir:{ir} m:{m}");
    }

    while jl != jr {
        let m = (jl + jr) / 2;
        stdout
            .write_fmt(format_args!("? 1 {n} {jl} {m}\n"))
            .unwrap();
        stdout.flush().unwrap();

        stdin.read_line(&mut buf).unwrap();
        let k = buf.trim_end().parse::<usize>().unwrap();
        buf.clear();

        if k == (m - jl + 1) {
            // (m + 1)..=jr に空きがある
            jl = m + 1;
        } else {
            // jl..=m に空きがある
            jr = m;
        }
        // eprintln!("jl:{jl} jr:{jr} m:{m}");
    }

    stdout.write_fmt(format_args!("! {il} {jl}\n")).unwrap();
    stdout.flush().unwrap();
}
