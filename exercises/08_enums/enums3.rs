struct Point {
    x: u64,
    y: u64,
}

enum Message {
    // TODO: Implement the message variant types based on their usage below.
    Move(Point),
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
    Resize { width: i32, height: i32 },
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.
        match message {
            Message::Move(point) => {
                self.move_position(point); // 使用 move_position 方法
            }
            Message::Echo(s) => {
                self.echo(s); // 使用 echo 方法
            }
            Message::ChangeColor(r, g, b) => {
                self.change_color(r.try_into().unwrap(), g.try_into().unwrap(), b.try_into().unwrap()); // 使用 change_color 方法，并解构参数
            }
            Message::Quit => {
                self.quit(); // 使用 quit 方法
            }
            Message::Resize { width, height } => {
                self.resize(width as u64, height as u64); // 使用 resize 方法，并转换类型
            }
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
