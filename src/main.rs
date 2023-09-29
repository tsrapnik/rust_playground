use self::theme::Theme;
use self::widget::Element;
use iced::{
    executor,
    widget::{button, column, text, text_input},
    Application, Command, Settings,
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

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                value: 0,
                s: "".into(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("test application")
    }

    // fn theme(&self) -> iced::Theme {}

    fn view(&self) -> Element<Message> {
        let text_input = text_input("Type something.", &self.s);
        let min_button_0 = button("-")
            .on_press(Message::DecrementPressed)
            .style(theme::Button::Primary);
        let min_button_1 = button("-")
            .on_press(Message::DecrementPressed)
            .style(theme::Button::Secondary);
        column!(
            text_input.on_input(|x| Message::InputReceived(x)),
            button("+")
                .on_press(Message::IncrementPressed)
                .style(theme::Button::Primary),
            text(self.value).size(50),
            min_button_0,
            min_button_1,
        )
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
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
        Command::none()
    }
}

fn main() {
    Counter::run(Settings::default()).unwrap();
}

mod widget {
    #![allow(dead_code)]
    use crate::{theme::Theme, Message};

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
    pub type TextInput<'a, Message> = iced::widget::TextInput<'a, Message, Renderer>;
    pub type Text<'a> = iced::widget::Text<'a, Renderer>;
    pub type Column<'a> = iced::widget::Column<'a, Message, Renderer>;
}

mod theme {
    use iced::{
        application,
        widget::{button, text, text_input},
        Color,
    };

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Theme;

    impl application::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: &Self::Style) -> application::Appearance {
            application::Appearance {
                background_color: Color::from_rgb(0.3, 0.3, 0.3),
                text_color: Color::from_rgb(0.8, 0.8, 0.8),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Button {
        #[default]
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Theme {
        type Style = Button;

        fn active(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Button::Primary => button::Appearance {
                    background: Some(Color::from_rgb(0.8, 0.3, 0.3).into()),
                    ..Default::default()
                },
                Button::Secondary => button::Appearance {
                    background: Some(Color::from_rgb(0.3, 0.8, 0.3).into()),
                    ..Default::default()
                },
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum TextInput {
        #[default]
        Default,
    }

    impl text_input::StyleSheet for Theme {
        type Style = TextInput;

        fn active(&self, style: &Self::Style) -> text_input::Appearance {
            text_input::Appearance {
                background: Color::from_rgb(0.3, 0.8, 0.3).into(),
                border_radius: 1.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.3, 0.8, 0.3).into(),
                icon_color: Color::from_rgb(0.3, 0.8, 0.3).into(),
            }
        }

        fn focused(&self, style: &Self::Style) -> text_input::Appearance {
            text_input::Appearance {
                background: Color::from_rgb(0.3, 0.8, 0.3).into(),
                border_radius: 1.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.3, 0.8, 0.3).into(),
                icon_color: Color::from_rgb(0.3, 0.8, 0.3).into(),
            }
        }

        fn placeholder_color(&self, style: &Self::Style) -> Color {
            Color::from_rgb(1.0, 0.0, 0.0).into()
        }

        fn value_color(&self, style: &Self::Style) -> Color {
            Color::from_rgb(0.0, 0.0, 0.0).into()
        }

        fn disabled_color(&self, style: &Self::Style) -> Color {
            Default::default()
        }

        fn selection_color(&self, style: &Self::Style) -> Color {
            Default::default()
        }

        fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
            text_input::Appearance {
                background: Color::from_rgb(0.3, 0.8, 0.3).into(),
                border_radius: 1.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.3, 0.8, 0.3).into(),
                icon_color: Color::from_rgb(0.3, 0.8, 0.3).into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Text {
        #[default]
        Default,
    }

    impl text::StyleSheet for Theme {
        type Style = Text;

        fn appearance(&self, style: Self::Style) -> text::Appearance {
            match style {
                Text::Default => text::Appearance {
                    color: Color::from_rgb(0.8, 0.8, 0.8).into(),
                },
            }
        }
    }
}
