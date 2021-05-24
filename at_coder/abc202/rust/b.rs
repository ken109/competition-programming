use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    }

    println!(
        "{}",
        s.iter()
            .rev()
            .map(|&c| match c {
                '6' => '9',
                '9' => '6',
                _ => c,
            })
            .collect::<String>()
    );
}
