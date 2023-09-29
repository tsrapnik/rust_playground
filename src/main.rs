use iced::{
    theme,
    widget::{button, column, radio, text, text_input},
    Color, Element, Sandbox, Settings,
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

    fn theme(&self) -> iced::Theme {
        let mut my_palette = theme::Theme::Dark.palette();
        my_palette.background = Color::from_rgb(1.0, 1.0, 1.0);
        my_palette.primary = Color::from_rgb(1.0, 0.0, 1.0);
        my_palette.danger = Color::from_rgb(1.0, 0.0, 0.0);
        my_palette.success = Color::from_rgb(0.0, 1.0, 0.0);
        my_palette.text = Color::from_rgb(0.2, 0.2, 0.2);

        theme::Theme::custom(my_palette)
    }

    fn view(&self) -> Element<Message> {
        let text_input = text_input("Type something.", &self.s);
        let min_button_0 = button("-")
            .on_press(Message::DecrementPressed)
            .style(theme::Button::Primary);
        let min_button_1 = button("-")
            .on_press(Message::DecrementPressed)
            .style(theme::Button::Secondary);
        let min_button_2 = button("-")
            .on_press(Message::DecrementPressed)
            .style(theme::Button::Positive);
        let min_button_3 = button("-")
            .on_press(Message::DecrementPressed)
            .style(theme::Button::Destructive);
        column!(
            text_input.on_input(|x| Message::InputReceived(x)),
            button("+")
                .on_press(Message::IncrementPressed)
                .style(theme::Button::Text),
            text(self.value).size(50),
            min_button_0,
            min_button_1,
            min_button_2,
            min_button_3,
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
