use std::num::NonZeroU32;
use std::sync::Arc;
#[cfg(debug_assertions)]
use std::time::Instant;

#[cfg(debug_assertions)]
use tezzera_trace::{event::TezzeraTrace, trace};

use tezzera_render::canvas::SkiaCanvas;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::event::{InputEvent, Key, MouseButton};

pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct TezzeraApp {
    config: AppConfig,
}

impl TezzeraApp {
    pub fn new() -> Self {
        Self {
            config: AppConfig {
                title: "Tezzera".to_string(),
                width: 800,
                height: 600,
            },
        }
    }

    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.config.title = t.into();
        self
    }

    pub fn size(mut self, w: u32, h: u32) -> Self {
        self.config.width = w;
        self.config.height = h;
        self
    }

    pub fn run<F>(self, paint_fn: F)
    where
        F: FnMut(&mut SkiaCanvas, &[InputEvent]),
    {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let mut app = AppState {
            config: self.config,
            paint_fn,
            window: None,
            surface: None,
            context: None,
            pending_events: Vec::new(),
            frame_counter: 0,
            cursor_x: 0.0,
            cursor_y: 0.0,
        };
        event_loop.run_app(&mut app).unwrap();
    }
}

impl Default for TezzeraApp {
    fn default() -> Self {
        Self::new()
    }
}

struct AppState<F> {
    config: AppConfig,
    paint_fn: F,
    window: Option<Arc<Window>>,
    context: Option<softbuffer::Context<Arc<Window>>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    pending_events: Vec<InputEvent>,
    frame_counter: u64,
    cursor_x: f32,
    cursor_y: f32,
}

impl<F: FnMut(&mut SkiaCanvas, &[InputEvent])> ApplicationHandler for AppState<F> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.config.width,
                self.config.height,
            ));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        self.context = Some(context);
        self.surface = Some(surface);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                let (width, height) = {
                    let size = self.window.as_ref().unwrap().inner_size();
                    (size.width, size.height)
                };
                if width == 0 || height == 0 {
                    return;
                }

                let surface = self.surface.as_mut().unwrap();
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                #[cfg(debug_assertions)]
                let frame = self.frame_counter;
                self.frame_counter += 1;

                #[cfg(debug_assertions)]
                let start = Instant::now();
                #[cfg(debug_assertions)]
                trace!(TezzeraTrace::FrameStart {
                    frame,
                    timestamp: start,
                });

                let mut canvas = SkiaCanvas::new(width, height);
                let events = std::mem::take(&mut self.pending_events);
                (self.paint_fn)(&mut canvas, &events);

                let mut buffer = surface.buffer_mut().unwrap();
                let pixels = canvas.pixels();
                for (i, pixel) in buffer.iter_mut().enumerate() {
                    let r = pixels[i * 4];
                    let g = pixels[i * 4 + 1];
                    let b = pixels[i * 4 + 2];
                    *pixel = ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
                }
                buffer.present().unwrap();

                #[cfg(debug_assertions)]
                {
                    let duration = start.elapsed();
                    let dropped = duration.as_secs_f64() * 1000.0 > 16.667;
                    trace!(TezzeraTrace::FrameEnd {
                        frame,
                        duration,
                        dropped,
                    });
                }

                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::Resized(size) => {
                self.pending_events.push(InputEvent::WindowResized {
                    width: size.width,
                    height: size.height,
                });
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_x = position.x as f32;
                self.cursor_y = position.y as f32;
                self.pending_events.push(InputEvent::MouseMove {
                    x: self.cursor_x,
                    y: self.cursor_y,
                });
            }

            WindowEvent::MouseInput { state, button, .. } => {
                let btn = match button {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    _ => return,
                };
                let (x, y) = (self.cursor_x, self.cursor_y);
                let ev = match state {
                    ElementState::Pressed  => InputEvent::MouseDown { x, y, button: btn },
                    ElementState::Released => InputEvent::MouseUp   { x, y, button: btn },
                };
                self.pending_events.push(ev);
            }

            WindowEvent::KeyboardInput { event, .. } => {
                let key = match event.physical_key {
                    PhysicalKey::Code(code) => match code {
                        KeyCode::Enter => Key::Enter,
                        KeyCode::Escape => Key::Escape,
                        KeyCode::Space => Key::Space,
                        KeyCode::Backspace => Key::Backspace,
                        KeyCode::Tab => Key::Tab,
                        KeyCode::ArrowUp => Key::ArrowUp,
                        KeyCode::ArrowDown => Key::ArrowDown,
                        KeyCode::ArrowLeft => Key::ArrowLeft,
                        KeyCode::ArrowRight => Key::ArrowRight,
                        _ => {
                            if let Some(text) = &event.text {
                                if let Some(c) = text.chars().next() {
                                    Key::Char(c)
                                } else {
                                    return;
                                }
                            } else {
                                return;
                            }
                        }
                    },
                    _ => return,
                };
                let ev = match event.state {
                    ElementState::Pressed => InputEvent::KeyDown { key },
                    ElementState::Released => InputEvent::KeyUp { key },
                };
                self.pending_events.push(ev);

                if let (ElementState::Pressed, Some(text)) = (event.state, event.text) {
                    for c in text.chars() {
                        self.pending_events.push(InputEvent::Text { character: c });
                    }
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}
