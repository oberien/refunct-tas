use std::collections::HashMap;
use std::iter;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use iced::{color, Length, Point};
use iced::widget::{container, mouse_area, text, Stack};
use rebo::BoundFunctionValue;
use screenshot_ui::ScreenshotUiElement;

mod screenshot_ui;
mod backend;
mod keyboard_input_mapper;
pub mod rebo_elements;

pub use screenshot_ui::{Clipboard, ScreenshotUi};
pub use keyboard_input_mapper::Key;
pub use backend::Backend;
use crate::threads::ue::iced_ui::rebo_elements::{IcedWindow, IcedWindowMessage, IcedWindowState};

// pub type UiBackend = backend::TinySkiaBackend;
pub type UiBackend = backend::WgpuBackend;
pub type Ui = ScreenshotUi<UiBackend, ReboUi>;
pub type Theme = iced::Theme;
pub type Renderer = <UiBackend as Backend>::Renderer;
pub type Element<Msg = Message> = iced::Element<'static, Msg, Theme, Renderer>;

#[derive(Debug, Clone)]
pub enum Message {
    ReboFunction(BoundFunctionValue<()>),
    MouseMoved(Point),
    WindowMessage(IcedWindowMessage),
}

pub struct ReboUi {
    windows: Arc<Mutex<Vec<IcedWindow>>>,
    window_state: HashMap<String, IcedWindowState>,
    tx: Sender<BoundFunctionValue<()>>,
    mouse_pos: Point,
}
impl ReboUi {
    pub fn new(windows: Arc<Mutex<Vec<IcedWindow>>>, tx: Sender<BoundFunctionValue<()>>) -> Self {
        Self {
            windows,
            window_state: HashMap::new(),
            tx,
            mouse_pos: Point::new(0., 0.),
        }
    }
}

impl<B: Backend> ScreenshotUiElement<B> for ReboUi
where screenshot_ui::Element<B, Message>: From<Stack<'static, Message, Theme, Renderer>>
{
    type Message = Message;
    fn update(&mut self, message: Message) {
        match message {
            Message::WindowMessage(message) => {
                for window in self.windows.lock().unwrap().iter_mut() {
                    let state = self.window_state.entry(window.id.clone()).or_default();
                    window.update(state, &message);
                }
            }
            Message::MouseMoved(point) => {
                let delta = point - self.mouse_pos;
                self.mouse_pos = point;
                for window in self.windows.lock().unwrap().iter_mut() {
                    let state = self.window_state.entry(window.id.clone()).or_default();
                    if let Some(function) = window.mouse_moved(state, delta) {
                        self.tx.send(function).unwrap();
                    }
                }
            }
            Message::ReboFunction(function) => self.tx.send(function).unwrap(),
        }
    }

    fn view(&self) -> screenshot_ui::Element<B, Message> {
        let lock = self.windows.lock().unwrap();
        let element: screenshot_ui::Element<B, Message> = Stack::with_children(
            lock.iter().map(|window| {
                window.view()
            }).chain(iter::once(
                mouse_area(container(text("")).width(Length::Fill).height(Length::Fill))
                    .on_move(Message::MouseMoved)
                    .into()
            ))
        ).width(Length::Fill).height(Length::Fill).into();
        element.explain(color!(0xff0000))
    }
}
