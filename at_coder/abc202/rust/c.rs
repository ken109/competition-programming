use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut count: Vec<u64> = vec![0; n];

    for v in c {
        count[b[v - 1] - 1] += 1;
    }

    println!("{}", a.iter().map(|&v| count[v - 1]).sum::<u64>());
}
