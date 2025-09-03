use iced::{color, Length, Vector};
use iced::widget::{button, container, pin, row, text, vertical_space, Column, Row, column, mouse_area};
use rebo::{BoundFunctionValue, TypedFunctionValue};
use crate::threads::ue::iced_ui::{Element, Message, Renderer, Theme};

#[derive(rebo::ExternalType, Debug, Clone)]
pub struct IcedWindow {
    pub id: String,
    pub title: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub element: IcedElement,
    pub on_move: TypedFunctionValue<fn(f32, f32)>,
    pub on_resize: TypedFunctionValue<fn(f32, f32)>,
}
impl IcedWindow {
    pub fn update(&self, state: &mut IcedWindowState, message: &IcedWindowMessage) {
        match message {
            IcedWindowMessage::TitlePressed(id) => if *id == self.id {
                state.title_pressed = true
            },
            IcedWindowMessage::ResizePressed(id) => if *id == self.id {
                state.resize_pressed = true
            },
            IcedWindowMessage::WindowReleased => {
                state.title_pressed = false;
                state.resize_pressed = false;
            },
        }
    }
    #[must_use]
    pub fn mouse_moved(&self, state: &mut IcedWindowState, delta: Vector) -> Option<BoundFunctionValue<()>> {
        if state.title_pressed {
            Some(self.on_move.clone().bind((self.x + delta.x, self.y + delta.y)))
        } else if state.resize_pressed {
            Some(self.on_resize.clone().bind((self.width + delta.x, self.height + delta.y)))
        } else {
            None
        }
    }
    pub fn view(&self) -> Element {
        pin(
            container(
                column![
                    self.element.view(),
                    vertical_space(),
                    container(
                        Element::from(row![
                            mouse_area(
                                container(
                                    text(self.title.clone())
                                ).center_x(Length::Fill),
                            ).on_press(IcedWindowMessage::TitlePressed(self.id.clone())),
                            mouse_area(
                                text("+")
                            ).on_press(IcedWindowMessage::ResizePressed(self.id.clone()))
                        ]).map(Message::WindowMessage)
                    ).style(|_| container::Style::default().background(color!(0xaaaaaa)))
                    .width(Length::Fill)
                ]
            ).width(self.width).height(self.height)
        ).x(self.x).y(self.y)
        .into()
    }
}
#[derive(Default, Debug)]
pub struct IcedWindowState {
    title_pressed: bool,
    resize_pressed: bool,
}
#[derive(Debug, Clone)]
pub enum IcedWindowMessage {
    // window-id
    TitlePressed(String),
    // window-id
    ResizePressed(String),
    WindowReleased,
}

#[derive(rebo::ExternalType, Debug, Clone)]
pub enum IcedElement {
    Button(IcedButton),
    Text(IcedText),
    Row(IcedRow),
    Column(IcedColumn),
}
impl IcedElement {
    pub fn view(&self) -> Element {
        match self {
            IcedElement::Button(e) => e.view(),
            IcedElement::Text(e) => e.view(),
            IcedElement::Row(e) => e.view(),
            IcedElement::Column(e) => e.view(),
        }
    }
}

#[derive(rebo::ExternalType, Debug, Clone)]
pub struct IcedButton {
    pub label: String,
    pub on_press: TypedFunctionValue<fn()>,
}
impl IcedButton {
    pub fn view(&self) -> Element {
        button(text(self.label.clone()))
            .on_press(Message::ReboFunction(self.on_press.clone().bind(())))
            .into()
    }
}

#[derive(rebo::ExternalType, Debug, Clone)]
pub struct IcedText {
    pub text: String,
}
impl IcedText {
    pub fn view(&self) -> Element {
        text(self.text.clone())
            .into()
    }
}

#[derive(rebo::ExternalType, Debug, Clone)]
pub struct IcedRow {
    pub elements: Vec<IcedElement>,
}
impl IcedRow {
    pub fn view(&self) -> Element {
        self.elements.iter()
            .map(IcedElement::view)
            .collect::<Row<Message, Theme, Renderer>>()
            .into()
    }
}

#[derive(rebo::ExternalType, Debug, Clone)]
pub struct IcedColumn {
    pub elements: Vec<IcedElement>,
}
impl IcedColumn {
    pub fn view(&self) -> Element {
        self.elements.iter()
            .map(IcedElement::view)
            .collect::<Column<Message, Theme, Renderer>>()
            .into()
    }
}
