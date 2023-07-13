enum Message {
    ChangeColor(u8, u8, u8),
    Quit,
    Echo(String),
    Move(Point),
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(r, g, b) => {
                self.change_color((r, g, b));
            }
            Message::Quit => {
                self.quit();
            }
            Message::Echo(s) => {
                self.echo(s);
            }
            Message::Move(p) => {
                self.move_position(p);
            }
        }
    }
}
fn main() {
    
}