use iced::{Center};
use iced::widget::{button, column, text};
use screenshot_ui::{Element, ScreenshotUiElement};

mod screenshot_ui;
mod keyboard_input_mapper;

pub use screenshot_ui::{ScreenshotUi, Clipboard};
pub use keyboard_input_mapper::Key;

pub type Ui = ScreenshotUi<Counter>;

#[derive(Default)]
pub struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl ScreenshotUiElement for Counter {
    type Message = Message;
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
            .padding(20)
            .align_x(Center)
            .into()
    }
}
