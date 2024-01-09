use iced::{
    executor,
    widget::{row, text, button, container},
    Application, Command, Settings,
};
use crate::ui::style;
use crate::ui::widget::Element;

pub mod ui;

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = style::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Plugins Search")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> Element<Message> {
        container(
            container(
                row![
                    button(text("primary"))
                        .style(style::Button::Primary),
                    button(text("secondary"))
                        .style(style::Button::Primary)
                ]
                    .spacing(10),
            )
                .padding(20)
                .style(style::Container::Transparent),
        )
            .center_x()
            .center_y()
            .into()
    }
}
