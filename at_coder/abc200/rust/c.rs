use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        ar: [usize; n]
    }

    let mut b = HashMap::<usize, u64>::new();

    for i in 0..ar.len() {
        if b.contains_key(&(ar[i] % 200)) {
            b.insert(ar[i] % 200, *b.get(&(ar[i] % 200)).unwrap() + 1);
        } else {
            b.insert(ar[i] % 200, 1);
        }
    }

    println!("{}", b.values().map(|&v| v * (v - 1) / 2).sum::<u64>())
}
