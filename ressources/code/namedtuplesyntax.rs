struct Complex(f64, f64);

fn main() {
    let z = Complex(1.0, 2.0);
    println!("Complex: ({}, {})",
        z.0, z.1);
}