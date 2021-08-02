use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars
    }

    let c = x
        .iter()
        .map(|v| v.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    if c[0] == c[1] && c[1] == c[2] && c[2] == c[3] {
        println!("Weak");
    } else {
        let mut weak = true;
        for i in 0..3 {
            if !(c[i + 1] == c[i] + 1 || (c[i] == 9 && c[i + 1] == 0)) {
                weak = false;
            }
        }
        if weak {
            println!("Weak")
        } else {
            println!("Strong")
        }
    }
}
