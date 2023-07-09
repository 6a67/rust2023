enum Direction {
    Up,         // 0
    Down,       // 1
    Left = 10,  // 10
    Right,      // 11
}
impl Direction {
    fn is_up(self) -> bool {
        self as i32 ==
            Direction::Up as i32
    }
}
fn main() {
    let up = Direction::Up;
    let down = Direction::Down;

    println!("Is up? {}", up.is_up());
    println!("Is down? {}", down.is_up());
}