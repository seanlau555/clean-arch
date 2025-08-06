enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main(){
    let go = Direction::Up;
    match go {
        Direction::Left => println!("go left");
    }
}

