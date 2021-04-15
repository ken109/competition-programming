use proconio::input;
use std::process::exit;

fn main() {
    input! {
        n: String
    }

    for i in 0..n.len() {
        let cs = format!("{}{}", String::from("0").repeat(i), n);
        let rev = cs.chars().rev().collect::<String>();

        if &cs[0..cs.len() / 2] == &rev[0..cs.len() / 2] {
            println!("Yes");
            exit(0);
        }
    }

    println!("No");
}
