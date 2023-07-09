let option = Some(5);
let other = false;

if let Some(value) = option {
    println!("Value: {}", value);
} else if other {
    println!("Other is true");
} else {
    println!("Option is None");
}

let mut vec = vec![1, 2, 3, 4, 5];
while let Some(value) = vec.pop() {
    println!("Value: {}", value);
}

let a = [1, 2, 3, 4, 5];
for (i, num) in a.iter().enumerate() {
    println!("{}: {}", i, num);
}