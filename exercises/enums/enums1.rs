// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit = 0,
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
