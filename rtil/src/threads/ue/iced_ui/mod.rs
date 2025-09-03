use std::collections::HashMap;
use std::{iter, mem, thread};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::sync::mpsc::{Receiver, Sender};
use iced::{color, keyboard, Event, Length, Point};
use iced::mouse::{Button, Interaction};
use iced::widget::{container, mouse_area, text, Stack};
use rebo::BoundFunctionValue;
use screenshot_ui::ScreenshotUiElement;
use crate::native::UTexture2D;
use crate::threads::ue::iced_ui::keyboard_input_mapper::KeyboardState;
use crate::threads::ue::iced_ui::rebo_elements::{IcedWindow, IcedWindowMessage, IcedWindowState};
use crate::threads::ue::iced_ui::screenshot_ui::ScreenshotUi;
use crate::watch;

mod screenshot_ui;
mod backend;
mod keyboard_input_mapper;
pub mod rebo_elements;

pub use screenshot_ui::Clipboard;
pub use keyboard_input_mapper::Key;
pub use backend::Backend;

// type UiBackend = backend::TinySkiaBackend;
type UiBackend = backend::WgpuBackend;
type Theme = iced::Theme;
type Renderer = <UiBackend as Backend>::Renderer;
type Element<Msg = Message> = iced::Element<'static, Msg, Theme, Renderer>;

#[derive(Debug, Clone)]
pub enum Message {
    ReboFunction(BoundFunctionValue<()>),
    MouseMoved(Point),
    WindowMessage(IcedWindowMessage),
}

enum InputEvent {
    Resize {
        width: u32,
        height: u32,
    },
    KeyPressed(keyboard::Event, Option<keyboard::Event>),
    KeyReleased(keyboard::Event, Option<keyboard::Event>),
    MouseMoved {
        screen_x: u32,
        screen_y: u32,
    },
    MouseButtonPressed(Button),
    MouseButtonReleased(Button),
    /// delta
    MouseWheel(f32),
}

pub struct UeTexture {
    pub texture: UTexture2D,
    pub interaction: Interaction,
}
pub struct ReboUi {
    ue_texture: Arc<Mutex<UeTexture>>,
    windows_tx: watch::Sender<Vec<IcedWindow>>,
    ui_event_rx: Receiver<BoundFunctionValue<()>>,
    input_event_tx: Sender<InputEvent>,
    keyboard_state: KeyboardState,
}
fn transparent_texture(width: u32, height: u32) -> UTexture2D {
    UTexture2D::create_with_pixelformat(&vec![0; width as usize * height as usize * 4], width.try_into().unwrap(), height.try_into().unwrap(), UiBackend::PIXEL_FORMAT)
}
impl ReboUi {
    pub fn start() -> Self {
        const DEFAULT_WIDTH: u32 = 1920;
        const DEFAULT_HEIGHT: u32 = 1080;
        let (windows_tx, windows_rx) = watch::channel();
        let ue_texture = Arc::new(Mutex::new(UeTexture {
            texture: transparent_texture(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            interaction: Interaction::default(),
        }));
        let (ui_event_tx, ui_event_rx) = mpsc::channel();
        let (input_event_tx, input_event_rx) = mpsc::channel();

        let state = ReboUiThreadState {
            screenshot_ui: ScreenshotUi::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            ue_texture: Arc::clone(&ue_texture),
            current_ue_texture: transparent_texture(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            windows_rx,
            window_state: HashMap::new(),
            ui_event_tx,
            input_event_rx,
            mouse_pos: Point::new(0., 0.),
        };
        state.start_thread();

        Self {
            ue_texture,
            windows_tx,
            ui_event_rx,
            input_event_tx,
            keyboard_state: KeyboardState::new(),
        }
    }

    pub fn next_ui_event(&self) -> Option<BoundFunctionValue<()>> {
        self.ui_event_rx.try_recv().ok()
    }

    pub fn set_windows(&self, windows: Vec<IcedWindow>) {
        self.windows_tx.send(windows).unwrap();
    }

    pub fn texture(&self) -> MutexGuard<'_, UeTexture> {
        self.ue_texture.lock().unwrap()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.input_event_tx.send(InputEvent::Resize { width, height }).unwrap();
    }

    pub fn key_pressed(&mut self, key: Key) -> Key {
        let (key, evt1, evt2) = self.keyboard_state.key_pressed(key);
        self.input_event_tx.send(InputEvent::KeyPressed(evt1, evt2)).unwrap();
        key
    }
    pub fn key_released(&mut self, key: Key) -> Key {
        let (key, evt1, evt2) = self.keyboard_state.key_released(key);
        self.input_event_tx.send(InputEvent::KeyReleased(evt1, evt2)).unwrap();
        key
    }

    pub fn mouse_moved(&mut self, screen_x: u32, screen_y: u32) {
        self.input_event_tx.send(InputEvent::MouseMoved { screen_x, screen_y }).unwrap();
    }

    pub fn mouse_button_pressed(&mut self, button: Button) {
        self.input_event_tx.send(InputEvent::MouseButtonPressed(button)).unwrap();
    }
    pub fn mouse_button_released(&mut self, button: Button) {
        self.input_event_tx.send(InputEvent::MouseButtonReleased(button)).unwrap();
    }
    pub fn mouse_wheel(&mut self, delta: f32) {
        self.input_event_tx.send(InputEvent::MouseWheel(delta)).unwrap();
    }
}

struct ReboUiThreadState {
    screenshot_ui: ScreenshotUi<UiBackend, Message>,
    ue_texture: Arc<Mutex<UeTexture>>,
    current_ue_texture: UTexture2D,
    windows_rx: watch::Receiver<Vec<IcedWindow>>,
    window_state: HashMap<String, IcedWindowState>,
    ui_event_tx: Sender<BoundFunctionValue<()>>,
    input_event_rx: Receiver<InputEvent>,
    mouse_pos: Point,
}
impl ReboUiThreadState {
    fn start_thread(mut self) {
        thread::spawn(move || {
            loop {
                let windows = self.windows_rx.read_consume().unwrap();

                // handle input events
                for event in self.input_event_rx.try_iter() {
                    match event {
                        InputEvent::Resize { width, height } => self.screenshot_ui.resize(width, height),
                        InputEvent::KeyPressed(evt1, evt2) | InputEvent::KeyReleased(evt1, evt2) => {
                            self.screenshot_ui.event(Event::Keyboard(evt1));
                            if let Some(evt2) = evt2 {
                                self.screenshot_ui.event(Event::Keyboard(evt2));
                            }
                        }
                        InputEvent::MouseMoved { screen_x, screen_y } => self.screenshot_ui.mouse_moved(screen_x, screen_y),
                        InputEvent::MouseButtonPressed(button) => self.screenshot_ui.mouse_button_pressed(button),
                        InputEvent::MouseButtonReleased(button) => self.screenshot_ui.mouse_button_released(button),
                        InputEvent::MouseWheel(delta) => self.screenshot_ui.mouse_wheel(delta),
                    }
                }

                // draw
                let width_changed = u32::try_from(self.current_ue_texture.width()).unwrap() != self.screenshot_ui.width();
                let height_changed = u32::try_from(self.current_ue_texture.height()).unwrap() != self.screenshot_ui.height();
                if width_changed || height_changed {
                    self.current_ue_texture = transparent_texture(self.screenshot_ui.width(), self.screenshot_ui.height());
                }
                let mut element = ReboUiElement {
                    windows: &windows,
                    window_state: &mut self.window_state,
                    mouse_pos: &mut self.mouse_pos,
                    ui_event_tx: &self.ui_event_tx,
                };

                // let interaction = self.screenshot_ui.draw_into(&mut element, &mut *self.current_ue_texture.as_mut_slice());
                let (interaction, buffer) = self.screenshot_ui.draw(&mut element);
                self.current_ue_texture.as_mut_slice().copy_from_slice(&buffer);
                let mut lock = self.ue_texture.lock().unwrap();
                mem::swap(&mut lock.texture, &mut self.current_ue_texture);
                lock.interaction = interaction;
            }
        });
    }
}

struct ReboUiElement<'a> {
    windows: &'a Vec<IcedWindow>,
    window_state: &'a mut HashMap<String, IcedWindowState>,
    mouse_pos: &'a mut Point,
    ui_event_tx: &'a Sender<BoundFunctionValue<()>>,
}

impl<'a, B: Backend> ScreenshotUiElement<B> for ReboUiElement<'a>
where screenshot_ui::Element<B, Message>: From<Stack<'static, Message, Theme, Renderer>>
{
    type Message = Message;
    fn update(&mut self, message: Message) {
        match message {
            Message::WindowMessage(message) => {
                for window in self.windows {
                    let state = self.window_state.entry(window.id.clone()).or_default();
                    window.update(state, &message);
                }
            }
            Message::MouseMoved(point) => {
                let delta = point - *self.mouse_pos;
                *self.mouse_pos = point;
                for window in self.windows {
                    let state = self.window_state.entry(window.id.clone()).or_default();
                    if let Some(function) = window.mouse_moved(state, delta) {
                        self.ui_event_tx.send(function).unwrap();
                    }
                }
            }
            Message::ReboFunction(function) => self.ui_event_tx.send(function).unwrap(),
        }
    }

    fn view(&self) -> screenshot_ui::Element<B, Message> {
        let element: screenshot_ui::Element<B, Message> = Stack::with_children(
            self.windows.iter().map(|window| {
                window.view()
            }).chain(iter::once(
                mouse_area(container(text("")).width(Length::Fill).height(Length::Fill))
                    .on_move(Message::MouseMoved)
                    .on_release(Message::WindowMessage(IcedWindowMessage::WindowReleased))
                    .into()
            ))
        ).width(Length::Fill).height(Length::Fill).into();
        element.explain(color!(0xff0000))
    }
}
