use crate::graphics::Graphics;
use crate::input::{InputEvent, InputState};
use crate::pixel_buf::PixBufMutView;
use crate::vectors::Vec2;
use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::{ExternalError, NotSupportedError};
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, Fullscreen, WindowBuilder};

pub type WinitWindow = winit::window::Window;
pub type Key = winit::event::VirtualKeyCode;

pub fn time() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to find time since the epoch")
}

#[allow(unused_variables)]
pub trait CanvasApp {
    fn setup(&mut self, window: &mut Window);
    fn render(&mut self, g: &mut Graphics, window: &mut Window);
    fn every_second(&mut self, window: &mut Window) {}
    fn input_event(&mut self, event: InputEvent, window: &mut Window) {}
    fn closing(&mut self) {}
}

pub struct Window {
    pub winit: WinitWindow,
    pub input: InputState,

    pub tracked_fps: u32,

    pub should_close: bool,
}
impl Window {
    pub fn get_pos(&self) -> Result<Vec2<i32>, NotSupportedError> {
        self.winit
            .outer_position()
            .map(|pos| Vec2::new(pos.x, pos.y))
    }
    pub fn set_pos(&self, pos: Vec2<i32>) {
        self.winit
            .set_outer_position(PhysicalPosition::new(pos.x, pos.y));
    }
    pub fn center_pos(&self) -> Option<()> {
        let mon_size = self.monitor_size()?.map(|e| e as i32);
        self.set_pos(mon_size / 2 - self.size().map(|e| e as i32) / 2);
        Some(())
    }

    pub fn monitor_size(&self) -> Option<Vec2<u32>> {
        let size = self.winit.current_monitor()?.size();
        Some(Vec2::new(size.width, size.height))
    }

    pub fn set_cursor_pos(&self, pos: Vec2<f32>) -> Result<(), ExternalError> {
        self.winit
            .set_cursor_position(PhysicalPosition::new(pos.x, pos.y))
    }

    pub fn set_size(&mut self, size: Vec2<u32>) {
        self.winit.set_inner_size(PhysicalSize::new(size.x, size.y));
    }
    pub fn size(&self) -> Vec2<u32> {
        let size = self.winit.inner_size();
        Vec2::new(size.width, size.height)
    }
    #[inline(always)]
    pub fn width(&self) -> u32 {
        self.winit.inner_size().width
    }
    #[inline(always)]
    pub fn height(&self) -> u32 {
        self.winit.inner_size().height
    }

    #[inline(always)]
    pub fn set_title(&mut self, title: &str) {
        self.winit.set_title(title);
    }

    pub fn set_fullscreen(&mut self, fs: bool) {
        let fs = if fs {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };
        self.winit.set_fullscreen(fs)
    }
    pub fn is_fullscreen(&self) -> bool {
        self.winit.fullscreen().is_some()
    }
    pub fn toggle_fullscreen(&mut self) {
        self.set_fullscreen(!self.is_fullscreen())
    }

    pub fn set_cursor_grab(&self, grab: bool) -> Result<(), ExternalError> {
        self.winit.set_cursor_grab(if grab {
            CursorGrabMode::Locked
        } else {
            CursorGrabMode::None
        })
    }
    pub fn set_cursor_vis(&self, vis: bool) {
        self.winit.set_cursor_visible(vis)
    }
}

pub struct CanvasConfig {
    pub title: String,
    pub size: Vec2<u32>,
    pub max_frames: f32,
    pub max_buffer_resizes: f32,
    pub fullscreen: bool,
}
impl CanvasConfig {
    pub fn new() -> Self {
        Self {
            title: String::from("canvas window"),
            size: Vec2::new(800, 600),
            max_frames: 60.0,
            max_buffer_resizes: 60.0,
            fullscreen: false,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }
    pub fn with_size(mut self, size: Vec2<u32>) -> Self {
        self.size = size;
        self
    }
    pub fn with_max_frames(mut self, m: f32) -> Self {
        self.max_frames = m;
        self
    }
    pub fn with_max_buffer_resized(mut self, m: f32) -> Self {
        self.max_buffer_resizes = m;
        self
    }
    pub fn fullscreen(mut self, fs: bool) -> Self {
        self.fullscreen = fs;
        self
    }
}

pub fn run_canvas_app(mut app: impl CanvasApp + 'static, config: CanvasConfig) -> ! {
    let CanvasConfig {
        title,
        size,
        max_frames,
        max_buffer_resizes,
        fullscreen,
    } = config;
    let fullscreen = if fullscreen {
        Some(Fullscreen::Borderless(None))
    } else {
        None
    };

    let event_loop = EventLoop::new();

    let winit = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(size.x, size.y))
        .with_title(title)
        .with_fullscreen(fullscreen)
        .build(&*event_loop)
        .unwrap();

    let mut window = Window {
        winit,
        input: InputState::default(),
        tracked_fps: 0,
        should_close: false,
    };
    window.center_pos();

    app.setup(&mut window);

    let mut pixels = Pixels::new(
        size.x,
        size.y,
        SurfaceTexture::new(size.x, size.y, &window.winit),
    )
    .unwrap();
    let mut buffer_size = size;

    let mut started_closing = false;

    let mut frames_this_second = 0;

    let mut last_buffer_resize = Instant::now();
    let mut last_stat_update = Instant::now();
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, flow| {
        let instant_now = Instant::now();

        // update stats
        if instant_now.duration_since(last_stat_update).as_millis() > 1000 {
            last_stat_update = instant_now;
            window.tracked_fps = frames_this_second;
            frames_this_second = 0;
            app.every_second(&mut window);
        }

        let can_draw =
            instant_now.duration_since(last_frame).as_millis() > (1000.0 / max_frames) as u128;
        let can_resize_buffer = instant_now.duration_since(last_buffer_resize).as_millis()
            > (1000.0 / max_buffer_resizes) as u128;

        // handle event
        match event {
            Event::RedrawRequested(_) => {
                let win_size = window.size();

                if can_draw {
                    if buffer_size != win_size
                        && can_resize_buffer
                        && win_size.x > 0
                        && win_size.y > 0
                    {
                        pixels.resize_buffer(win_size.x, win_size.y);
                        buffer_size = win_size;
                        last_buffer_resize = instant_now;
                    }

                    let mut graphics = PixBufMutView {
                        bytes: pixels.get_frame(),
                        size: buffer_size,
                    }
                    .create_graphics();
                    app.render(&mut graphics, &mut window);

                    pixels.render().unwrap();

                    last_frame = instant_now;
                    frames_this_second += 1;
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(_size) => {
                    // Surface says to call `resize_surface` on a window resize event
                    let win_size = window.size();
                    pixels.resize_surface(win_size.x, win_size.y);
                }
                WindowEvent::CloseRequested => {
                    window.should_close = true;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if input.state == ElementState::Pressed {
                            window.input.pressed_keys.insert(key);
                            app.input_event(InputEvent::KeyPressed(key), &mut window);
                        } else {
                            window.input.pressed_keys.remove(&key);
                            app.input_event(InputEvent::KeyReleased(key), &mut window);
                        }
                    }
                }
                WindowEvent::CursorMoved { position: pos, .. } => {
                    window.input.cursor_pos = Vec2::new(pos.x, pos.y);
                    // TODO send update to app
                }
                _ => {}
            },
            _ => {}
        };

        window.winit.request_redraw();

        // check closing condition
        if window.should_close {
            *flow = ControlFlow::Exit;
            if !started_closing {
                app.closing();
            }
            started_closing = true;
        }
    })
}
