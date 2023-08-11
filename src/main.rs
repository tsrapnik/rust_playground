use iced::{
    widget::{button, column, radio, text},
    Element, Sandbox, Settings,
};
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("test application")
    }

    fn view(&self) -> Element<Message> {
        column!(
            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("-").on_press(Message::DecrementPressed),
            radio(String::from("label"), true, None, |_x| {
                Message::DecrementPressed
            }),
        )
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}

fn main() {
    Counter::run(Settings::default()).unwrap();
}
