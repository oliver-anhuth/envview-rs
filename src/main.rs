use iced::widget::{self, Column};
use iced::{Element, Result, Sandbox, Settings};
use std::env;

fn main() -> Result {
    Environment::run(Settings::default())
}

#[derive(Default, Debug)]
struct Environment;

impl Sandbox for Environment {
    type Message = ();

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("envview")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<'_, Self::Message> {
        let mut vars = Column::new();
        for (name, value) in env::vars() {
            vars = vars.push(widget::row![
                widget::text(name),
                widget::text(" = "),
                widget::text(value)
            ]);
        }
        widget::scrollable(vars).into()
    }
}
