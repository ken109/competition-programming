use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        cs: Chars
    }

    let s: String = cs.iter().collect();

    println!("{}", s.matches("ZONe").count())
}
