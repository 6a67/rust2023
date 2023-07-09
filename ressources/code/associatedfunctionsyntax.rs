struct Circle;

impl Circle {
    fn area(r: f64) -> f64 {
        std::f64::consts::PI * r * r
    }
}
fn main() {
    let area = Circle::area(2.0);
    println!("The area is {}.", area);
}