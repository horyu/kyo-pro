#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        q: usize,
        ttss: [(usize, Chars); q],
    };
    let mut xx = Trie::<26, 'a'>::new();
    let mut yy: Vec<Vec<char>> = vec![];
    for (t, s) in ttss {
        if t == 1 {
            xx.insert_autoid(&s);
            yy.retain(|y| !xx.start_with(&y));
        } else {
            if !xx.start_with(&s) {
                yy.push(s);
            }
        }
        let rs = yy.len();
        println!("{rs}");
        if q < 5 {
            eprintln!("{yy:?}");
        }
    }
}

// https://algo-logic.info/trie-tree/
#[allow(unused)]
#[derive(Clone, Debug)]
struct Node<const CHAR_SIZE: usize> {
    next: Vec<Option<usize>>,
    accept: Vec<usize>,
    c: usize,
    common: usize,
}

impl<const CHAR_SIZE: usize> Node<CHAR_SIZE> {
    fn new(c: usize) -> Self {
        Self {
            next: vec![None; CHAR_SIZE],
            accept: vec![],
            c,
            common: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Trie<const CHAR_SIZE: usize, const BASE: char> {
    nodes: Vec<Node<CHAR_SIZE>>,
}

#[allow(unused)]
impl<const CHAR_SIZE: usize, const BASE: char> Trie<CHAR_SIZE, BASE> {
    fn new() -> Self {
        let nodes = vec![Node::new(0)];
        Self { nodes }
    }

    fn insert(&mut self, word: &[char], word_id: usize) {
        let mut node_id = 0;
        for &ch in word {
            let c = (ch as u8 - BASE as u8) as usize;
            if self.nodes[node_id].next[c].is_none() {
                let new_id = self.nodes.len();
                self.nodes[node_id].next[c] = Some(new_id);
                self.nodes.push(Node::new(c));
            }
            self.nodes[node_id].common += 1;
            node_id = self.nodes[node_id].next[c].unwrap();
        }
        self.nodes[node_id].common += 1;
        self.nodes[node_id].accept.push(word_id);
    }

    fn insert_autoid(&mut self, word: &[char]) {
        let word_id = self.nodes[0].common;
        self.insert(word, word_id);
    }

    fn search(&self, word: &[char], prefix: bool) -> bool {
        let mut node_id = 0;
        for &ch in word {
            let c = (ch as u8 - BASE as u8) as usize;
            match self.nodes[node_id].next[c] {
                Some(next_id) => node_id = next_id,
                None => return false,
            }
        }
        if prefix {
            true
        } else {
            !self.nodes[node_id].accept.is_empty()
        }
    }

    fn start_with(&self, prefix: &[char]) -> bool {
        self.search(prefix, true)
    }

    fn count(&self) -> usize {
        self.nodes[0].common
    }

    fn size(&self) -> usize {
        self.nodes.len()
    }
}
