use proconio::input;

fn main() {
    input! {
        (a, b, c): (u32, u32, u32)
    }

    if a.pow(2) + b.pow(2) < c.pow(2) {
        println!("Yes")
    } else {
        println!("No")
    }
}
