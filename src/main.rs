use iced::{
    widget::{container, text_editor},
    Element, Sandbox, Settings,
};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        "editor".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Edit(action) => self.content.edit(action),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let input = text_editor(&self.content).on_edit(Message::Edit);
        container(input).padding(10).into()
    }
}
