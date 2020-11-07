#[cfg(test)]
mod tests {

    #[derive(Debug)]
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move {x: i32, y: i32},
        Write(String),
    }

    enum BoardGameTurn {
        Move {squares: i32},
        Pass,
    }

    #[test]
    fn test() {
        let x = Message::Move {x:3, y: 3};
        let y = BoardGameTurn::Move {squares: 4};

        let m = Message::Write("Hello, world".to_string());

        let m = foo("Hello, world".to_string());

        let v = vec!["hello", "world"];
        let v1: Vec<Message> = v.iter().map(|e| Message::Write(e.to_string())).collect();


    }

    fn foo(x: String) -> Message {
        Message::Write(x)
    }

    fn process_color_change(msg: Message) {

    }
}
