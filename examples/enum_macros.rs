use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    println!("up = {:?}", up);

    let left: Direction = 10.into();
    println!("left = {:?}", left);
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}
