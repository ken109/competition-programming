use proconio::input;

fn main() {
    input! {
        s: String
    }

    println!("{}", s.matches("ZONe").count())
}
