use iced_tiny_skia::{Renderer as TinySkiaRenderer, Renderer};
use iced_wgpu::{Engine as WgpuEngine, Renderer as WgpuRenderer, wgpu, ScreenshotBuffers as WgpuScreenshotBuffers};
use iced::{Color, Font, Pixels, Size, Rectangle};
use iced::advanced::graphics::Viewport;
use crate::native::EPixelFormat;

pub trait Backend: 'static {
    const PIXEL_FORMAT: EPixelFormat;
    type Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer;

    fn create(width: u32, height: u32) -> Self;
    fn renderer(&mut self) -> &mut Self::Renderer;
    fn size_changed(&mut self, width: u32, height: u32);
    fn draw_into(&mut self, width: u32, height: u32, out_buf: &mut [u8]);
}

#[allow(unused)]
pub struct TinySkiaBackend {
    renderer: TinySkiaRenderer,
    clip_mask: tiny_skia::Mask,
    image_buffer: Vec<u8>,
}

impl Backend for TinySkiaBackend {
    const PIXEL_FORMAT: EPixelFormat = EPixelFormat::B8G8R8A8;
    type Renderer = TinySkiaRenderer;

    fn create(width: u32, height: u32) -> Self {
        TinySkiaBackend {
            renderer: Renderer::new(Font::default(), Pixels::from(24)),
            clip_mask: tiny_skia::Mask::new(width, height).unwrap(),
            image_buffer: vec![0u8; width as usize * height as usize * 4],
        }
    }

    fn renderer(&mut self) -> &mut Self::Renderer {
        &mut self.renderer
    }

    fn size_changed(&mut self, width: u32, height: u32) {
        self.image_buffer.resize(width as usize * height as usize * 4, 0u8);
        self.image_buffer.shrink_to_fit();
        self.clip_mask = tiny_skia::Mask::new(width, height).unwrap();
    }

    fn draw_into(&mut self, width: u32, height: u32, out_buf: &mut [u8]) {
        self.image_buffer.fill(0);
        self.renderer.draw(
            &mut tiny_skia::PixmapMut::from_bytes(
                &mut self.image_buffer,
                width,
                height,
            )
                .expect("Create offscreen pixel map"),
            &mut self.clip_mask,
            &Viewport::with_physical_size(Size::new(width, height), 1.),
            &[Rectangle::with_size(Size::new(
                width as f32,
                height as f32,
            ))],
            Color::TRANSPARENT,
        );
        out_buf.copy_from_slice(&self.image_buffer);
    }
}

#[allow(unused)]
pub struct WgpuBackend {
    renderer: WgpuRenderer,
    screenshot_buffers: WgpuScreenshotBuffers,
    image_buffer: Vec<u8>,
}

impl Backend for WgpuBackend {
    const PIXEL_FORMAT: EPixelFormat = EPixelFormat::R8G8B8A8;
    type Renderer = WgpuRenderer;

    fn create(width: u32, height: u32) -> Self {
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
                    .request_device(&wgpu::DeviceDescriptor {
                        required_features: wgpu::Features::CLEAR_TEXTURE,
                        ..Default::default()
                    })
                    .await
                    .expect("Request device");

                (
                    adapter,
                    device,
                    queue,
                )
            });
        let engine = WgpuEngine::new(
            &adapter,
            device.clone(),
            queue.clone(),
            wgpu::TextureFormat::Rgba8UnormSrgb,
            None,
        );
        let mut renderer = WgpuRenderer::new(
            engine,
            Font::default(),
            Pixels::from(24),
        );
        let screenshot_buffers = renderer.create_screenshot_buffers("rtil-screenshot-ui", &Viewport::with_physical_size(Size::new(width, height), 1.));
        WgpuBackend {
            renderer,
            screenshot_buffers,
            image_buffer: vec![0u8; width as usize * height as usize * 4],
        }
    }

    fn renderer(&mut self) -> &mut Self::Renderer {
        &mut self.renderer
    }

    fn size_changed(&mut self, width: u32, height: u32) {
        self.image_buffer.resize(width as usize * height as usize * 4, 0u8);
        self.image_buffer.shrink_to_fit();
        self.screenshot_buffers = self.renderer.create_screenshot_buffers("rtil-screenshot-ui", &Viewport::with_physical_size(Size::new(width, height), 1.));
    }

    fn draw_into(&mut self, width: u32, height: u32, out_buf: &mut [u8]) {
        self.image_buffer.fill(0);
        self.renderer.screenshot_into(
            &Viewport::with_physical_size(Size::new(width, height), 1.),
            Color::TRANSPARENT,
            &mut self.screenshot_buffers,
            &mut self.image_buffer,
        );
        out_buf.copy_from_slice(&self.image_buffer);
        // let vec = self.renderer.screenshot(
        //     &Viewport::with_physical_size(Size::new(width, height), 1.),
        //     Color::TRANSPARENT,
        // );
        // out_buf.copy_from_slice(&vec);
    }
}
