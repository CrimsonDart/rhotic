use std::{num::NonZeroU32, fmt::{Display, Debug}, collections::HashMap, rc::Rc, cell::Cell};
use softbuffer::{Context, Surface};
use winit::{
    window::WindowBuilder,
    event_loop::EventLoop, error::OsError, event::{MouseScrollDelta, ElementState, VirtualKeyCode}, dpi::PhysicalSize};

use crate::state::{application::State, widgets::{button::Button, background::{Background, self}, glyph::Glyph}};

use super::{render, Point, types::Pixel, vulkan::get_graphics};

pub fn start_event_loop() -> Result<(), OsError> {

    get_graphics();











    let event_loop = EventLoop::new();

    let window =
        WindowBuilder::new()
        .with_title("Rhotic Text Editor")
        .with_window_icon(None)
        .with_min_inner_size(PhysicalSize::new(200, 200))
        .build(&event_loop)?;

    let context = unsafe {
        Context::new(&window)
    }.unwrap();

    let mut surface = unsafe {
        Surface::new(&context, &window)
    }.unwrap();

    let mut state = State::new();

    state.widgets.background.fit_to_window({
        let w = window.inner_size();
        Point::new(w.width, w.height)
    });

    state.widgets.layer1.push(Rc::new(Glyph::new(50, 50, 'A', u32::MAX)));

    event_loop.run(move |event, _window_target, control_flow| {

        control_flow.set_poll();

        use winit::event::Event::*;
        match event {

            MainEventsCleared => {
                state.advance();
                window.request_redraw();
            },

            WindowEvent {
                window_id,
                event
            } => if window_id == window.id() {

                use winit::event::WindowEvent::*;

                match event {
                    Resized(_) => {
                        state.widgets.background.fit_to_window({
                            let w = window.inner_size();
                            Point::new(w.width, w.height)
                        });

                        window.request_redraw();
                    },

                    CloseRequested => {
                        control_flow.set_exit();
                    },

                    Focused(is) => {

                        state.is_focused = is;

                        #[cfg(debug_assertions)]
                        println!("Focused: {}", is);
                    },

                    CursorMoved { device_id: _, position, modifiers: _ } => {
                        state.input.mouse_position = Point::new(position.x as u32, position.y as u32);
                    },

                    MouseWheel { device_id: _, delta, phase: _, modifiers: _ } => {
                        state.input.scroll_delta = Some(delta);
                    },

                    MouseInput { device_id: _, state: pressed, button, modifiers: _ } => {

                        use winit::event::MouseButton::*;

                        let press = if pressed == ElementState::Pressed {
                            ButtonState::Pressed
                        } else {
                            ButtonState::Released
                        };

                        match button {
                            Left => {
                                state.input.m1 = press;
                            },
                            Right => {
                                state.input.m2 = press;
                            },
                            Middle => {
                                state.input.m3 = press;
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },

            DeviceEvent { device_id: _, event } => if let winit::event::DeviceEvent::Key(key) = event {

                let press = if key.state == ElementState::Pressed {
                    ButtonState::Pressed
                } else {
                    ButtonState::Released
                };

                let keys = &mut state.input.keys;

                match key.virtual_keycode {
                    Some(k) => {
                        keys.insert(KeyType::Keycode(k), press);
                    },
                    None => {
                        keys.insert(KeyType::Scancode(key.scancode), press);
                    }
                }
            },

            RedrawRequested(window_id) if window_id == window.id() => {

                let size = {
                    let size = window.inner_size();
                    Point::new(size.width, size.height)
                };

                surface.resize(
                    NonZeroU32::new(size.x).unwrap(),
                    NonZeroU32::new(size.y).unwrap()
                ).unwrap();

                let buffer = surface.buffer_mut().unwrap();
                render(buffer, size, &state);
            },
            _ => {}
        }
    });
}

#[derive(PartialEq, Debug)]
pub struct Input {
    pub mouse_position: Pixel,
    pub scroll_delta: Option<MouseScrollDelta>,

    pub m1: ButtonState,
    pub m2: ButtonState,
    pub m3: ButtonState,

    pub keys: HashMap<KeyType, ButtonState>

}

impl Input {
    pub fn advance_state(&mut self) {

        self.scroll_delta = None;

        self.m1 = self.m1.advance_state();
        self.m2 = self.m2.advance_state();
        self.m3 = self.m3.advance_state();

        for (_key, value) in self.keys.iter_mut() {
            *value = value.advance_state();
        }

    }
}

impl Default for Input {
    fn default() -> Self {
        use ButtonState::*;

        Self { mouse_position: Point::new(0, 0),
                     scroll_delta: None,
                     m1: Depressed,
                     m2: Depressed,
                     m3: Depressed,
               keys: HashMap::new()
        }
    }
}

#[cfg(debug_assertions)]
impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "position: {}, {}", self.mouse_position.x, self.mouse_position.y)?;

        match self.scroll_delta {
            Some(MouseScrollDelta::LineDelta(a, b)) => {
                write!(f, " | Scroll Line: {}, {}", a, b)?;
            },
            Some(MouseScrollDelta::PixelDelta(p)) => {
                write!(f, " | Scroll Pixel: {}, {}", p.x, p.y)?;
            }
            _ => {}
        }

        if self.m1 != ButtonState::Depressed {
            write!(f, " | M1: {}", self.m1)?;
        }
        if self.m2 != ButtonState::Depressed {
            write!(f, " | M2: {}", self.m2)?;
        }
        if self.m3 != ButtonState::Depressed {
            write!(f, " | M3: {}", self.m3)?;
        }

        write!(f, " | ")?;

        for value in self.keys.iter() {
            if value.1 != &ButtonState::Depressed {

                match value.0 {
                    KeyType::Scancode(n) => {
                        write!(f, "SC({:?}): {} | ", n, value.1)?;
                    },
                    KeyType::Keycode(key) => {
                        write!(f, "{:?}: {} | ", key, value.1)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum KeyType {
    Scancode(u32),
    Keycode(VirtualKeyCode)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonState {
    Pressed,
    Held,
    Released,
    Depressed
}

impl ButtonState {
    fn advance_state(self) -> Self {
        use ButtonState::*;
        match self {
            Pressed => Held,
            Released => Depressed,
            _ => self
        }
    }
}

impl Display for ButtonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ButtonState::*;

        write!(f, "{}", match self {
            Pressed => "P",
            Held => "H",
            Released => "R",
            Depressed => "D"
        })
    }
}
