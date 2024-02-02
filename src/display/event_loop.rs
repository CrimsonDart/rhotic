use std::{num::NonZeroU32, fmt::{Display, Debug}, time::Instant};
use softbuffer::{Context, Surface};
use winit::{
    window::WindowBuilder,
    event_loop::EventLoop, event::{MouseScrollDelta, ElementState}, dpi::PhysicalSize, keyboard::PhysicalKey};

use crate::state::application::State;

use super::{render, Point, types::Pixel};

pub fn start_event_loop() -> anyhow::Result<()> {

    let event_loop = EventLoop::new()?;

    let window =
        WindowBuilder::new()
        .with_title("Rhotic Text Editor")
        .with_window_icon(None)
        .with_min_inner_size(PhysicalSize::new(200, 200))
        .build(&event_loop)?;

    let context = Context::new(&window).unwrap();

    let mut surface = Surface::new(&context, &window).unwrap();

    let mut state = State::new()?;

    event_loop.run(|event, elwt| {

        elwt.set_control_flow(winit::event_loop::ControlFlow::Poll);

        use winit::event::Event::*;
        match event {

            AboutToWait => {
                state.advance();
                window.request_redraw();
            },

            WindowEvent {
                window_id,
                event
            } => if window_id == window.id() {

                use winit::event::WindowEvent::*;

                match event {
                    RedrawRequested if window_id == window.id() => {

                        let size = {
                            let size = window.inner_size();
                            Point::new(size.width, size.height)
                        };

                        surface.resize(
                            NonZeroU32::new(size.x).unwrap(),
                            NonZeroU32::new(size.y).unwrap()
                        ).unwrap();

                        let buffer = surface.buffer_mut().unwrap();
                        render(buffer, size, &mut state);
                    },
                    Resized(_) => {
                        window.request_redraw();
                    },

                    CloseRequested => {
                        elwt.exit();
                    },

                    Focused(is) => {

                        state.is_focused = is;

                        #[cfg(debug_assertions)]
                        println!("Focused: {}", is);
                        window.request_redraw();
                    },

                    CursorMoved { device_id: _, position, } => {
                        state.input.mouse_position = Point::new(position.x as u32, position.y as u32);
                        window.request_redraw();
                    },

                    MouseWheel { device_id: _, delta, phase: _} => {
                        state.input.scroll_delta = Some(delta);
                        window.request_redraw();
                    },

                    MouseInput { device_id: _, state: pressed, button } => {

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
                        window.request_redraw();
                    },
                    KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                        println!("{:?}", event.physical_key);
                        if event.state == ElementState::Pressed {
                            if let Some(s) = event.logical_key.to_text() {
                                state.input.text.push_str(s);
                                if event.repeat {
                                    for (k, v) in state.input.keys.iter_mut() {
                                        if *k == event.physical_key {
                                            if let ButtonState::Held(t) = *v {
                                                *v = ButtonState::Echo(t);
                                            }
                                        }
                                    }
                                } else {
                                    state.input.keys.push((event.physical_key, ButtonState::Pressed));
                                }
                            }
                        } else {
                            for (k,v) in state.input.keys.iter_mut() {
                                if *k == event.physical_key {
                                    *v = ButtonState::Released;
                                }
                            }
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    })?;
    Ok(())
}

#[derive(PartialEq, Debug)]
pub struct Input {
    pub mouse_position: Pixel,
    pub scroll_delta: Option<MouseScrollDelta>,

    pub m1: ButtonState,
    pub m2: ButtonState,
    pub m3: ButtonState,

    pub keys: Vec<(PhysicalKey, ButtonState)>,
    pub text: String,
}

impl Input {
    pub fn advance_state(&mut self) {

        self.scroll_delta = None;

        self.m1 = self.m1.advance_state();
        self.m2 = self.m2.advance_state();
        self.m3 = self.m3.advance_state();

        let mut new_vec = Vec::new();

        for (key, value) in self.keys.iter_mut() {
            if *value != ButtonState::Released || *value != ButtonState::Depressed {
                if *value == ButtonState::Pressed {
                    new_vec.push((key.clone(), ButtonState::Held(Instant::now())));
                } else if let ButtonState::Held(t) = *value {
                    new_vec.push((key.clone(), ButtonState::Held(t)));
                } else if let ButtonState::Echo(t) = *value {
                    new_vec.push((key.clone(), ButtonState::Held(t)));
                }
            }
        }
        self.keys = new_vec;
        self.text = String::new();
    }
}

impl Default for Input {
    fn default() -> Self {
        use ButtonState::*;

        Self {
            mouse_position: Point::new(0, 0),
            scroll_delta: None,
            m1: Depressed,
            m2: Depressed,
            m3: Depressed,

            keys: Vec::new(),
            text: String::new()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonState {
    Pressed,
    Echo(Instant),
    Held(Instant),
    Released,
    Depressed
}

impl ButtonState {
    fn advance_state(self) -> Self {
        use ButtonState::*;
        match self {
            Pressed => Held(Instant::now()),
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
            Echo(_) => "E",
            Held(_) => "H",
            Released => "R",
            Depressed => "D"
        })
    }
}
