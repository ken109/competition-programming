use proconio::input;

fn main() {
    input! {
        mut n: u64,
        k: usize
    }

    for _ in 0..k {
        if n % 200 == 0 {
            n = n / 200
        } else {
            n = String::from(n.to_string() + "200").parse().unwrap()
        }
    }

    println!("{}", n)
}
