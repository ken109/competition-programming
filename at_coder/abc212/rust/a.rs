use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8
    }

    if a == 0 {
        println!("Silver")
    } else if b == 0 {
        println!("Gold")
    } else {
        println!("Alloy")
    }
}
