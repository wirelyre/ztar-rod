#![allow(unused_imports)]
use glium::glutin::{
    ContextBuilder, DeviceEvent, ElementState, Event, EventsLoop, MouseButton, Window,
    WindowBuilder, WindowEvent,
};
use glium::{Display, Surface};
use std::env;
use std::fs::File;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use ztar_rod::mod_dir::ModDir;
use ztar_rod::render::{Camera, Map, Renderer, Scene};

#[derive(Default)]
struct State {
    camera: Camera,
    closed: bool,
    is_clean: bool,
    held: u8, // 0b1111 WASD
    mouse_down: bool,
}

fn main() {
    let map_name = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("run as cargo --bin fly <map name>"));
    let mod_dir = ModDir::open(Path::new("./mod"));

    let mut events_loop = EventsLoop::new();
    let wb = WindowBuilder::new()
        .with_dimensions((1024, 768).into())
        .with_title("Map Viewer");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &events_loop).unwrap();

    let renderer = Renderer::new(&display);
    let scene = Scene::new(&display, &mod_dir, &map_name).unwrap();
    let mut state = State::default();

    while !state.closed {
        if !state.is_clean {
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 0.5, 1.0), 1.0);
            renderer.render(&mut target, &scene, &state.camera);
            target.finish().unwrap();

            state.is_clean = true;
        }

        sleep(Duration::from_millis(16));

        events_loop.poll_events(|ev| state.handle(&display, &ev));

        state.tick();
    }
}

use std::ops::Deref;

impl State {
    fn handle(&mut self, display: &glium::backend::glutin::Display, ev: &Event) {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => self.closed = true,
                WindowEvent::Refresh => self.is_clean = false,

                WindowEvent::KeyboardInput { input, .. } => match input.state {
                    ElementState::Pressed => self.held |= held_bits_from_scancode(input.scancode),
                    ElementState::Released => self.held &= !held_bits_from_scancode(input.scancode),
                },

                WindowEvent::MouseInput {
                    state,
                    button: MouseButton::Right,
                    ..
                } => {
                    self.mouse_down = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    };

                    let gl_window = display.gl_window();
                    let win: &Window = gl_window.window();

                    win.grab_cursor(self.mouse_down).unwrap();
                    win.hide_cursor(self.mouse_down);
                }

                _ => (),
            },

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if self.mouse_down {
                    self.is_clean = false;
                    self.camera.pan(cgmath::Deg(delta.0 as f32 / 5.0).into());
                    self.camera.tilt(cgmath::Deg(delta.1 as f32 / 5.0).into());
                }
            }

            _ => (),
        }
    }

    fn tick(&mut self) {
        if self.held != 0 {
            self.is_clean = false;
        }

        match self.held & 0b1010 {
            0b1000 => self.camera.dolly(-10.0), // W
            0b0010 => self.camera.dolly(10.0),  // S
            _ => (),
        }

        match self.held & 0b0101 {
            0b0100 => self.camera.truck(-10.0), // A
            0b0001 => self.camera.truck(10.0),  // D
            _ => (),
        }
    }
}

fn held_bits_from_scancode(code: u32) -> u8 {
    match code {
        13 => 0b1000, // W
        0 => 0b0100,  // A
        1 => 0b0010,  // S
        2 => 0b0001,  // D
        _ => 0,
    }
}
