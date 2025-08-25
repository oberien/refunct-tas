use std::fmt::Debug;
use std::mem;
use clipboard::{ClipboardContext, ClipboardProvider};
use iced::{Event, keyboard, mouse, Point, Size, window};
use iced::advanced::clipboard::Kind;
use iced::advanced::renderer::Style;
use iced::mouse::{Button, Cursor, Interaction, ScrollDelta};
use iced::theme::Theme;
use iced_runtime::core::input_method;
use iced_runtime::keyboard::Modifiers;
use iced_runtime::user_interface::{Cache, State};
use iced_runtime::UserInterface;
use crate::threads::ue::iced_ui::backend::Backend;
use crate::threads::ue::iced_ui::keyboard_input_mapper::{Key, KeyboardState};

pub type Element<B, Message> = iced::Element<'static, Message, Theme, <B as Backend>::Renderer>;

pub struct ScreenshotUi<B: Backend, T: ScreenshotUiElement<B>> {
    backend: B,
    element: T,
    cache: Cache,
    events: Vec<Event>,
    messages: Vec<T::Message>,
    width: u32,
    height: u32,
    keyboard_state: KeyboardState,
    cursor: Cursor,
}
unsafe impl<B: Backend, T: ScreenshotUiElement<B> + Send> Send for ScreenshotUi<B, T> {}

impl<B: Backend, T: ScreenshotUiElement<B> + Default + 'static> ScreenshotUi<B, T> {
    pub fn new(width: u32, height: u32) -> Self {
        Self::create(T::default(), width, height)
    }
}
impl<B: Backend, T: ScreenshotUiElement<B> + 'static> ScreenshotUi<B, T> {
    pub fn create(element: T, width: u32, height: u32) -> Self {
        ScreenshotUi {
            element,
            backend: B::create(width, height),
            cache: Cache::new(),
            events: vec![
                Event::Window(window::Event::Opened { position: Some(Point::new(0., 0.)), size: Size::new(width as f32, height as f32) }),
                Event::InputMethod(input_method::Event::Closed),
                Event::Window(window::Event::Focused),
                Event::Mouse(mouse::Event::CursorEntered),
                Event::Keyboard(keyboard::Event::ModifiersChanged(Modifiers::empty())),
            ],
            messages: Vec::new(),
            width,
            height,
            keyboard_state: KeyboardState::new(),
            cursor: Cursor::default(),
            // screenshot_buffers,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width == width && self.height == height {
            return;
        }
        self.width = width;
        self.height = height;
        self.backend.size_changed(width, height);
        self.events.push(Event::Window(window::Event::Resized(Size::new(width as f32, height as f32))));
    }

    pub fn key_pressed(&mut self, key: Key) -> Key {
        let (key, evt1, evt2) = self.keyboard_state.key_pressed(key);
        self.events.push(Event::Keyboard(evt1));
        if let Some(evt2) = evt2 {
            self.events.push(Event::Keyboard(evt2));
        }
        key
    }
    pub fn key_released(&mut self, key: Key) -> Key {
        let (key, evt1, evt2) = self.keyboard_state.key_released(key);
        self.events.push(Event::Keyboard(evt1));
        if let Some(evt2) = evt2 {
            self.events.push(Event::Keyboard(evt2));
        }
        key
    }

    pub fn mouse_moved(&mut self, screen_x: u32, screen_y: u32) {
        let position = Point::new(screen_x as f32, screen_y as f32);
        self.cursor = Cursor::Available(position);
        self.events.push(Event::Mouse(mouse::Event::CursorMoved { position }));
    }

    pub fn mouse_button_pressed(&mut self, button: Button) {
        self.events.push(Event::Mouse(mouse::Event::ButtonPressed(button)))
    }
    pub fn mouse_button_released(&mut self, button: Button) {
        self.events.push(Event::Mouse(mouse::Event::ButtonReleased(button)))
    }
    pub fn mouse_wheel(&mut self, delta: f32) {
        self.events.push(Event::Mouse(mouse::Event::WheelScrolled { delta: ScrollDelta::Lines { x: 0., y: delta } }))
    }

    fn build_user_interface(&mut self) -> UserInterface<'static, T::Message, Theme, B::Renderer> {
        UserInterface::build(
            self.element.view(),
            Size::new(self.width as f32, self.height as f32),
            mem::take(&mut self.cache),
            self.backend.renderer(),
        )
    }

    pub fn draw(&mut self) -> (Interaction, Vec<u8>) {
        let mut vec = vec![0u8; self.width as usize * self.height as usize * 4];
        let interaction = self.draw_into(&mut vec);
        (interaction, vec)
    }

    pub fn draw_into(&mut self, buf: &mut [u8]) -> Interaction {
        let mut user_interface = self.build_user_interface();

        let (state, _event_statuses) = user_interface.update(
            &self.events,
            self.cursor,
            self.backend.renderer(),
            &mut Clipboard,
            &mut self.messages,
        );

        if !self.messages.is_empty() {
            for message in self.messages.drain(..) {
                self.element.update(message);
            }
            self.cache = user_interface.into_cache();
            user_interface = self.build_user_interface();
        }

        user_interface.draw(
            self.backend.renderer(),
            &Theme::default(),
            &Style::default(),
            self.cursor,
        );

        self.cache = user_interface.into_cache();
        self.events.clear();

        self.backend.draw_into(self.width, self.height, buf);
        match state {
            State::Outdated => Interaction::None,
            State::Updated { mouse_interaction, .. } => mouse_interaction,
        }
    }
}

pub trait ScreenshotUiElement<B: Backend> {
    type Message: Debug;
    fn update(&mut self, message: Self::Message);
    fn view(&self) -> Element<B, Self::Message>;
    fn _view_borrowed(&self) -> iced::Element<'_, Self::Message, Theme, B::Renderer> {
        self.view()
    }
}

pub struct Clipboard;
impl Clipboard {
    pub fn get() -> Option<String> {
        let mut ctx: ClipboardContext = ClipboardProvider::new().ok()?;
        ctx.get_contents().ok()
    }
    pub fn set(content: String) {
        let mut ctx: ClipboardContext = match ClipboardProvider::new() {
            Ok(ctx) => ctx,
            Err(_) => return,
        };
        let _ = ctx.set_contents(content);
    }
}
impl iced::advanced::Clipboard for Clipboard {
    fn read(&self, _kind: Kind) -> Option<String> {
        Clipboard::get()
    }

    fn write(&mut self, _kind: Kind, contents: String) {
        Clipboard::set(contents)
    }
}
