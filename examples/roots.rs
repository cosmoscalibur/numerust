use numerust::roots::closed::{bisection, incremental};

fn quadratic(x: f32) -> f32 {
    x.powi(2) - 3.0
}

fn main() {
    println!(
        "incremental {:?}",
        incremental(quadratic, 0.0, 10.0, 0.001).unwrap()
    );
    println!(
        "bisection {:?}",
        bisection(quadratic, 0.0, 10.0, 0.001).unwrap()
    );
}
