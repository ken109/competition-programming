use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    println!("{}", 7 - a + 7 - b + 7 - c);
}
