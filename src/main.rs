use iced::widget::{self, Column};
use iced::{Result, Sandbox, Settings};
use std::env;

fn main() -> Result {
    Environment::run(Settings::default())
}

#[derive(Default, Debug)]
struct Environment;

#[derive(Copy, Clone, Debug)]
enum Action {}

impl Sandbox for Environment {
    type Message = Action;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("envview")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut col = Column::new();
        for (name, value) in env::vars() {
            col = col.push(widget::row![widget::text(name), widget::text(" = "), widget::text(value)]);
        }
        widget::scrollable(col).into()
    }
}
