use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!("{}", if n % 100 == 0 { n / 100 } else { n / 100 + 1 })
}
