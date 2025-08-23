use std::fmt::Debug;
use std::mem;
use image::RgbaImage;
use clipboard::{ClipboardContext, ClipboardProvider};
use iced::{Color, Event, Font, keyboard, mouse, Pixels, Point, Size, window};
use iced::advanced::clipboard::Kind;
use iced::mouse::{Button, Cursor, Interaction, ScrollDelta};
use iced_runtime::core::input_method;
use iced_runtime::keyboard::Modifiers;
use iced_runtime::user_interface::{Cache, State};
use iced_runtime::UserInterface;
use iced_wgpu::core::Theme;
use iced_wgpu::{Engine, Renderer, wgpu};
use iced_wgpu::core::renderer::Style;
use iced_wgpu::graphics::Viewport;
use crate::threads::ue::iced_ui::keyboard_input_mapper::{Key, KeyboardState};

pub type Element<Message> = iced::Element<'static, Message, Theme, Renderer>;

pub struct ScreenshotUi<T: ScreenshotUiElement> {
    element: T,
    renderer: Renderer,
    cache: Cache,
    events: Vec<Event>,
    messages: Vec<T::Message>,
    size: (u32, u32),
    keyboard_state: KeyboardState,
    cursor: Cursor,
}
unsafe impl<T: ScreenshotUiElement + Send> Send for ScreenshotUi<T> {}

impl<T: ScreenshotUiElement + Default + 'static> ScreenshotUi<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Self::create(T::default(), width, height)
    }
}
impl<T: ScreenshotUiElement + 'static> ScreenshotUi<T> {
    pub fn create(element: T, width: u32, height: u32) -> Self {
        // don't allow OpenGL due to a data race with eglMakeCurrent: https://stackoverflow.com/q/74085533
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::from_env().unwrap_or(wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::DX12),
            ..Default::default()
        });
        let (adapter, device, queue) =
            futures::executor::block_on(async {
                let adapter = wgpu::util::initialize_adapter_from_env_or_default(
                    &instance,
                    None,
                )
                    .await
                    .expect("Create adapter");

                let (device, queue) = adapter
                    .request_device(&wgpu::DeviceDescriptor::default())
                    .await
                    .expect("Request device");

                (
                    adapter,
                    device,
                    queue,
                )
            });
        let engine = Engine::new(
            &adapter,
            device.clone(),
            queue.clone(),
            wgpu::TextureFormat::Rgba8UnormSrgb,
            None,
        );
        let renderer = Renderer::new(
            engine,
            Font::default(),
            Pixels::from(24),
        );

        ScreenshotUi {
            element,
            renderer,
            cache: Cache::new(),
            events: vec![
                Event::Window(window::Event::Opened { position: Some(Point::new(0., 0.)), size: Size::new(width as f32, height as f32) }),
                Event::InputMethod(input_method::Event::Closed),
                Event::Window(window::Event::Focused),
                Event::Mouse(mouse::Event::CursorEntered),
                Event::Keyboard(keyboard::Event::ModifiersChanged(Modifiers::empty())),
            ],
            messages: Vec::new(),
            size: (width, height),
            keyboard_state: KeyboardState::new(),
            cursor: Cursor::default(),
        }
    }

    pub fn resize(&mut self, x: u32, y: u32) {
        if self.size == (x, y) {
            return;
        }
        self.size = (x, y);
        self.events.push(Event::Window(window::Event::Resized(Size::new(x as f32, y as f32))));
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

    fn build_user_interface(&mut self) -> UserInterface<'static, T::Message, Theme, Renderer> {
        UserInterface::build(
            self.element.view(),
            Size::new(self.size.0 as f32, self.size.1 as f32),
            mem::take(&mut self.cache),
            &mut self.renderer,
        )
    }

    pub fn draw(&mut self) -> (Interaction, RgbaImage) {
        let mut user_interface = self.build_user_interface();

        let (state, _event_statuses) = user_interface.update(
            &self.events,
            self.cursor,
            &mut self.renderer,
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
            &mut self.renderer,
            &Theme::default(),
            &Style::default(),
            self.cursor,
        );

        self.cache = user_interface.into_cache();
        self.events.clear();

        let buf = self.renderer.screenshot(
            &Viewport::with_physical_size(Size::new(self.size.0, self.size.1), 1.),
            Color::TRANSPARENT,
        );
        let interaction = match state {
            State::Outdated => Interaction::None,
            State::Updated { mouse_interaction, .. } => mouse_interaction,
        };
        (
            interaction,
            RgbaImage::from_vec(self.size.0, self.size.1, buf).unwrap(),
        )

        // TODO: clipboard
    }
}

pub trait ScreenshotUiElement {
    type Message: Debug;
    fn update(&mut self, message: Self::Message);
    fn view(&self) -> Element<Self::Message>;
    fn _view_borrowed(&self) -> iced::Element<'_, Self::Message, Theme, Renderer> {
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
