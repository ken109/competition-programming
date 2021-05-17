use proconio::input;
use proconio::marker::Chars;

use itertools::Itertools;
use std::collections::HashSet;
use std::process::exit;

fn main() {
    input! {s: Chars}

    let mut must: Vec<usize> = Vec::new();
    let mut ok: Vec<usize> = Vec::new();

    for (i, &c) in s.iter().enumerate() {
        if c == 'o' {
            must.push(i)
        }
        if c != 'x' {
            ok.push(i)
        }
    }

    if must.len() > 4 {
        println!("0");
        exit(0)
    }

    let mut ans_set: HashSet<Vec<usize>> = HashSet::new();

    for remain in ok.iter().combinations_with_replacement(4 - must.len()) {
        let mut combination = must.clone();
        combination.extend(remain);
        for permutation in combination.iter().permutations(4) {
            ans_set.insert(permutation.iter().map(|&v| *v).collect());
        }
    }

    println!("{:?}", ans_set.len());
}
