let opt = Some(5);
let number = 7;

match opt {
    Some(x) if x == number => {
        println!("x = {:?}", x);
    },
    _ => println!("Default case"),
}