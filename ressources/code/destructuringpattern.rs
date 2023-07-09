struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let triple = (0, -2, 3);
    let (x, y, z) = triple; 
    // x: 0, y: -2, z: 3

    match triple {
        (0, y, z) => println!("x: 0, y: {y}, z: {z}"),
        (1, ..) => println!("x is 1, rest doesn't matter"),
        (3, .., 5) => println!("x: 3, z: 5"),
        _ => println!("It doesn't matter what they are"),
    }

    let p = Point { x: 0, y: 7, z: 0 };
    match p {
        Point { x: 0, y, z } => println!("x: 0, y: {y}, z: {z}"),
        Point { x: 1, y: a, z: b } => println!("x: {x}, y: {a}, z: {b}"),
        Point { x, .. } => println!("x: {x}, rest doesn't matter"),
    }
}