#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move {x: i32, y: i32},
        Write(String),
    }


    fn quit() {}
    fn change_color(r: i32, g: i32, b: i32) {}
    fn move_cursor(x: i32, y: i32) {}

    fn process_message(msg: Message) {
        match msg {
            Message::Quit =>quit(),
            Message::ChangeColor(r, g, b) => change_color(r, g, b),
            Message::Move {x, y} => move_cursor(x, y),
            Message::Write(s) => println!("s is {}", s),
        }
    }

    #[test]
    fn test() {

        let x = 5;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            4 => println!("four"),
            _ => println!("something else")
        }

        let m = Message::ChangeColor(0, 0, 0);
        process_message(m);

    }
}
