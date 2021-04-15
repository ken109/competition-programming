use proconio::input;

fn main() {
    input! {
        r: f64,
        x: f64,
        y: f64,
    }

    let d = (x.powi(2) + y.powi(2)).sqrt();

    if d < r {
        println!("2");
    } else {
        println!("{}", (d / r).ceil() as usize);
    }
}
