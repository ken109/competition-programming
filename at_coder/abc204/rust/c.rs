use im_rc::HashMap;
use proconio::input;

type Map = HashMap<usize, Vec<usize>>;

fn calc(map: &Map, left: &mut Vec<bool>, start: usize) {
    if left[start - 1] {
        return;
    }
    left[start - 1] = true;
    for &v in map.get(&start).unwrap().iter() {
        calc(map, left, v);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abs: [(usize, usize); m]
    }

    let mut map: Map = HashMap::new();

    for i in 0..n {
        map.entry(i + 1).or_insert(Vec::new()).push(i + 1);
    }

    for ab in abs {
        map.entry(ab.0).or_insert(Vec::new()).push(ab.1);
    }

    let mut sum: usize = 0;
    for k in 1..=n {
        let mut left: Vec<bool> = vec![false; n];
        calc(&map, &mut left, k);
        sum += left.into_iter().filter(|v| *v).count();
    }

    println!("{}", sum);
}
