use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    println!(
        "{}",
        a.into_iter()
            .filter(|v| *v > 10)
            .map(|v| v - 10)
            .sum::<usize>()
    );
}
