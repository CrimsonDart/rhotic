use std::{num::NonZeroU32, fmt::{Display, Debug}, collections::HashMap};
use softbuffer::{Context, Surface};
use winit::{
    window::WindowBuilder,
    event_loop::EventLoop, error::OsError, event::{MouseScrollDelta, ElementState, VirtualKeyCode}};

use crate::state::application::State;

use super::{render, font::load_ttf, Rgba, Point};

pub fn start_event_loop() -> Result<(), OsError> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Rhotic Text Editor")
        .with_window_icon(None)
        .build(&event_loop)?
        ;

    let context = unsafe {
        Context::new(&window)
    }.unwrap();

    let mut surface = unsafe {
        Surface::new(&context, &window)
    }.unwrap();

    let mut state = State {
        font: load_ttf("assets/fonts/Inconsolata-Regular.ttf").unwrap(),
        display_text: String::from("EEEEEE"),
        background_color: Rgba::new(0,0,0,255),
        text_color: Rgba::new(255, 255, 255, 255),
        mouse_state: MouseState::default(),
        keyboard_state: KeyboardState::new(),
        is_focused: false
    };

    event_loop.run(move |event, _window_target, control_flow| {

        control_flow.set_wait();

        use winit::event::Event::*;
        match event {

            MainEventsCleared => state.advance(),

            WindowEvent {
                window_id,
                event
            } => if window_id == window.id() {

                use winit::event::WindowEvent::*;

                match event {
                    Resized(_) => {
                        window.request_redraw();
                    },

                    CloseRequested => {
                        control_flow.set_exit();
                    },

                    Focused(is) => {
                        println!("Focused: {}", is);
                    },

                    CursorMoved { device_id: _, position, modifiers: _ } => {
                        state.mouse_state.position = Point::new(position.x as u32, position.y as u32);
                    },

                    MouseWheel { device_id: _, delta, phase: _, modifiers: _ } => {
                        state.mouse_state.scroll_delta = Some(delta);
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
                                state.mouse_state.left_button = press;
                            },
                            Right => {
                                state.mouse_state.right_button = press;
                            },
                            Middle => {
                                state.mouse_state.middle_button = press;
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

                let keys = &mut state.keyboard_state.keys;

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

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                surface.resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap()
                ).unwrap();

                render(&mut surface, width, height, &state);
            },
            _ => {}
        }
    });
}

// Mouse Input handling

#[derive(Clone, PartialEq, Debug)]
pub struct MouseState {
    pub position: Point<u32>, // in pixel position (top left is 0,0)
    pub scroll_delta: Option<MouseScrollDelta>,

    pub left_button: ButtonState,
    pub right_button: ButtonState,
    pub middle_button: ButtonState,
}

impl MouseState {
    pub fn advance_state(&mut self) {

        self.scroll_delta = None;

        self.left_button = self.left_button.advance_state();
        self.right_button = self.right_button.advance_state();
        self.middle_button = self.middle_button.advance_state();

    }
}

impl Default for MouseState {

    fn default() -> Self {
        use ButtonState::*;

        MouseState { position: Point::new(0, 0),
                     scroll_delta: None,
                     left_button: Depressed,
                     right_button: Depressed,
                     middle_button: Depressed }
    }
}

impl Display for MouseState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let (scroll_type, amount) = match self.scroll_delta {
            Some(MouseScrollDelta::LineDelta(a, b)) => {
                ("Line", (a as f64, b as f64))
            },
            Some(MouseScrollDelta::PixelDelta(p)) => {
                ("Pixel", (p.x, p.y))
            },
            None => {
                ("None", (0.0, 0.0))
            }
        };

        write!(f, "position: {}, {} | Scroll {}: {}, {} | M1: {} | M2: {} | M3: {}",
               self.position.x, self.position.y,
               scroll_type, amount.0, amount.1,
               self.left_button,
               self.right_button,
               self.middle_button

        )
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeyboardState {
    keys: HashMap<KeyType, ButtonState>
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new()
        }
    }

    pub fn advance_state(&mut self) {

        for (_key, value) in self.keys.iter_mut() {
            *value = value.advance_state();
        }
    }
}

impl Display for KeyboardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

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
