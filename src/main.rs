use anyhow::Result;
use std::{error::Error, fmt, fmt::Display};

use iced::{
    widget::{button, column, radio, text, text_input},
    Element, Sandbox,
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
        let text_input = text_input("Type something.", &self.s);
        column!(
            text_input.on_input(|x| Message::InputReceived(x)),
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
    // Counter::run(Settings::default()).unwrap();
    println!("{:?}", error_wrapper(WhichError::First));
    println!("{:?}", error_wrapper(WhichError::Second));
    println!("{:?}", error_wrapper(WhichError::None));
}

#[derive(Debug)]
struct MyError1 {
    number: u32,
}

impl Display for MyError1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error number {}", self.number)
    }
}

impl Error for MyError1 {}

#[derive(Debug)]
struct MyError2 {
    number: u32,
}

impl Display for MyError2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error number {}", self.number)
    }
}

impl Error for MyError2 {}

fn error_or_not_1(throw_error: bool) -> Result<u32, MyError1> {
    if throw_error {
        Err(MyError1 { number: 4 })
    } else {
        Ok(8)
    }
}

fn error_or_not_2(throw_error: bool) -> Result<u32, MyError2> {
    if throw_error {
        Err(MyError2 { number: 6 })
    } else {
        Ok(10)
    }
}

enum WhichError {
    First,
    Second,
    None,
}

fn error_wrapper(which_error: WhichError) -> Result<u32> {
    let x = error_or_not_1(matches!(which_error, WhichError::First))?;
    let y = error_or_not_2(matches!(which_error, WhichError::Second))?;
    Ok(u32::max(x, y))
}
