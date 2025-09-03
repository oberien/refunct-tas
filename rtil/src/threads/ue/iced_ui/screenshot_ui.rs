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

pub type Element<B, Message> = iced::Element<'static, Message, Theme, <B as Backend>::Renderer>;

pub struct ScreenshotUi<B: Backend, Message> {
    backend: B,
    cache: Cache,
    events: Vec<Event>,
    messages: Vec<Message>,
    width: u32,
    height: u32,
    cursor: Cursor,
}
unsafe impl<B: Backend, Message> Send for ScreenshotUi<B, Message> {}

impl<B: Backend, Message> ScreenshotUi<B, Message> {
    pub fn new(width: u32, height: u32) -> Self {
        ScreenshotUi {
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
            cursor: Cursor::default(),
            // screenshot_buffers,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
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

    pub fn event(&mut self, event: Event) {
        self.events.push(event);
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

    fn build_user_interface<T>(&mut self, element: &T) -> UserInterface<'static, T::Message, Theme, B::Renderer>
        where T: ScreenshotUiElement<B, Message = Message>
    {
        UserInterface::build(
            element.view(),
            Size::new(self.width as f32, self.height as f32),
            mem::take(&mut self.cache),
            self.backend.renderer(),
        )
    }

    pub fn draw<T: ScreenshotUiElement<B, Message = Message>>(&mut self, element: &mut T) -> (Interaction, Vec<u8>) {
        let mut vec = vec![0u8; self.width as usize * self.height as usize * 4];
        let interaction = self.draw_into(element, &mut vec);
        (interaction, vec)
    }

    pub fn draw_into<T: ScreenshotUiElement<B, Message = Message>>(&mut self, element: &mut T, buf: &mut [u8]) -> Interaction {
        let mut user_interface = self.build_user_interface(element);

        let (state, _event_statuses) = user_interface.update(
            &self.events,
            self.cursor,
            self.backend.renderer(),
            &mut Clipboard,
            &mut self.messages,
        );

        if !self.messages.is_empty() {
            for message in self.messages.drain(..) {
                element.update(message);
            }
            self.cache = user_interface.into_cache();
            user_interface = self.build_user_interface(element);
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
