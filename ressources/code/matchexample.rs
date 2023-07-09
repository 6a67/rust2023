let number = 7;
match number {
    1 => println! ("One"),
    2 | 3 | 5 | 7 | 11 => {
        println! ("Prime")
    }, 
    13..=19 => println! ("Teen"),
    x => println! ("{x} not special"),
}
