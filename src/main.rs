use iced::{
    widget::{button, column, radio, text, text_input, text_input::StyleSheet},
    Element, Sandbox, Settings,
};
struct Counter {
    value: i32,
    s: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    InputReceived(String),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            s: "".into(),
        }
    }

    fn title(&self) -> String {
        String::from("test application")
    }

    fn view(&self) -> Element<Message> {
        column!(
            text_input("Type something.", &self.s).on_input(|x| Message::InputReceived(x)),
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
            Message::InputReceived(s) => {
                self.s = s;
            }
        }
    }
}

fn main() {
    Counter::run(Settings::default()).unwrap();
}
