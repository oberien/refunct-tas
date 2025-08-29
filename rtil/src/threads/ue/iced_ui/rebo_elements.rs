use iced::{color, Length, Vector};
use iced::widget::{button, container, pin, row, text, vertical_space, Column, Row, column, mouse_area};
use rebo::{BoundFunctionValue, TypedFunctionValue};
use crate::threads::ue::iced_ui::{Element, Message, Renderer, Theme};

#[derive(rebo::ExternalType, Debug)]
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
    pub fn update(&mut self, state: &mut IcedWindowState, message: &IcedWindowMessage) {
        if self.id != message.window_id {
            return;
        }
        match message.kind {
            IcedWindowMessageKind::TitlePressed => state.title_pressed = true,
            IcedWindowMessageKind::TitleReleased => state.title_pressed = false,
            IcedWindowMessageKind::ResizePressed => state.resize_pressed = true,
            IcedWindowMessageKind::ResizeReleased => state.resize_pressed = false,
        }
    }
    #[must_use]
    pub fn mouse_moved(&mut self, state: &mut IcedWindowState, delta: Vector) -> Option<BoundFunctionValue<()>> {
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
                            ).on_press(IcedWindowMessage { window_id: self.id.clone(), kind: IcedWindowMessageKind::TitlePressed })
                            .on_release(IcedWindowMessage { window_id: self.id.clone(), kind: IcedWindowMessageKind::TitleReleased }),
                            mouse_area(
                                text("+")
                            ).on_press(IcedWindowMessage { window_id: self.id.clone(), kind: IcedWindowMessageKind::ResizePressed })
                            .on_release(IcedWindowMessage { window_id: self.id.clone(), kind: IcedWindowMessageKind::ResizeReleased }),
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
pub struct IcedWindowMessage {
    window_id: String,
    kind: IcedWindowMessageKind,
}
#[derive(Debug, Clone, Copy)]
pub enum IcedWindowMessageKind {
    TitlePressed,
    TitleReleased,
    ResizePressed,
    ResizeReleased,
}

#[derive(rebo::ExternalType, Debug)]
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

#[derive(rebo::ExternalType, Debug)]
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

#[derive(rebo::ExternalType, Debug)]
pub struct IcedText {
    pub text: String,
}
impl IcedText {
    pub fn view(&self) -> Element {
        text(self.text.clone())
            .into()
    }
}

#[derive(rebo::ExternalType, Debug)]
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

#[derive(rebo::ExternalType, Debug)]
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
