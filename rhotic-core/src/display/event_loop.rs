use std::{num::NonZeroU32, fmt::{Display, Debug}, time::Instant, collections::HashMap, ops::IndexMut};
use enum_iterator::{Sequence, all};
use softbuffer::{Context, Surface, Buffer};
use winit::{
    window::{WindowBuilder, Window},
    event_loop::EventLoop, event::{MouseScrollDelta, ElementState}, dpi::PhysicalSize, keyboard::{PhysicalKey, KeyCode}};

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
                        use ButtonState::*;

                        let buf = match button {
                            Left => {
                                Key::M1
                            },
                            Right => {
                                Key::M2
                            },
                            Middle => {
                                Key::M3
                            },
                            _ => { return; }
                        };

                        state.input[buf] = if pressed == ElementState::Pressed {
                            Pressed(Instant::now())
                        } else if let Pressed(t) | Held(t) | Echo(t) | Released(t) = state.input[buf] {
                            Released(t)
                        } else {
                            Depressed
                        };

                        window.request_redraw();
                    },
                    KeyboardInput { device_id: _, event, is_synthetic: _ } => {

                        if event.state == ElementState::Pressed {
                            if let Some(s) = event.logical_key.to_text() {
                                state.input.text.push_str(s);
                            }
                        }

                        let key = if let PhysicalKey::Code(k) = event.physical_key {
                            k
                        } else {
                            return;
                        };

                        let key = if let Some(k) = get_keycode_name(key) {
                            k
                        } else {
                            return;
                        };

                        if event.state == ElementState::Pressed {
                            if event.repeat {
                                if let ButtonState::Held(t) = state.input[key] {
                                    state.input[key] = ButtonState::Echo(t);
                                    return;
                                }
                            }
                            state.input[key] = ButtonState::Pressed(Instant::now());
                            return;
                        }

                        state.input[key] = match state.input[key] {
                            ButtonState::Depressed => ButtonState::Depressed,
                            ButtonState::Echo(t) | ButtonState::Held(t) | ButtonState::Pressed(t) | ButtonState::Released(t) => {
                                ButtonState::Released(t)
                            }
                        };
                        window.request_redraw();
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
    pub array: [ButtonState; 85],

    pub text: String,
}

impl Input {
    pub fn advance_state(&mut self) {

        for i in 0..self.array.len() {
            self.array[i] = self.array[i].advance_state();
        }

        self.scroll_delta = None;
        self.text = String::new();
    }

    pub fn get_pressed_keys(&self) -> Vec<Key> {
        let mut out = Vec::new();

        for i in all::<Key>() {
            if self.array[i as usize].is_pressed() {
                out.push(i);
            }
        }

        out
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.array[key as usize].is_pressed()
    }

    pub fn is_any_key_pressed(&self, keys: &[Key]) -> bool {
        for key in keys {
            if self.array[*key as usize].is_pressed() {
                return true;
            }
        }
        false
    }
}

impl Default for Input {
    fn default() -> Self {
        use ButtonState::*;

        Self {
            mouse_position: Point::new(0, 0),
            scroll_delta: None,
            array: [Depressed; 85],
            text: String::new()
        }
    }
}

impl std::ops::Index<Key> for Input {
    type Output = ButtonState;
    fn index(&self, index: Key) -> &Self::Output {
        &self.array[index as usize]
    }
}

impl std::ops::IndexMut<Key> for Input {
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        &mut self.array[index as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonState {
    Pressed(Instant),
    Echo(Instant),
    Held(Instant),
    Released(Instant),
    Depressed
}

impl ButtonState {
    fn advance_state(mut self) -> Self {
        use ButtonState::*;
        self = match self {
            Depressed | Released(_) => Depressed,
            Pressed(t) | Held(t) | Echo(t) => Held(t)
        };
        self
    }

    pub fn is_pressed(&self) -> bool {
        use ButtonState::*;
        match self {
            Pressed(_) | Echo(_) | Held(_) => true,
            _ => false
        }
    }
}

impl Display for ButtonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ButtonState::*;

        write!(f, "{}", match self {
            Pressed(_) => "P",
            Echo(_) => "E",
            Held(_) => "H",
            Released(_) => "R",
            Depressed => "D"
        })
    }
}


const fn get_keycode_name(key: KeyCode) -> Option<Key> {
    use KeyCode::*;
    Some(match key {
        Backquote => Key::Grave,
        Backslash | IntlRo => Key::Backslash,
        BracketLeft => Key::Bracketleft,
        BracketRight | IntlBackslash => Key::Bracketright,
        Comma | NumpadComma => Key::Comma,
        IntlYen => Key::Yen,

        Digit0 | Numpad0 => Key::N0,
        Digit1 | Numpad1 => Key::N1,
        Digit2 | Numpad2 => Key::N2,
        Digit3 | Numpad3 => Key::N3,
        Digit4 | Numpad4 => Key::N4,
        Digit5 | Numpad5 => Key::N5,
        Digit6 | Numpad6 => Key::N6,
        Digit7 | Numpad7 => Key::N7,
        Digit8 | Numpad8 => Key::N8,
        Digit9 | Numpad9 => Key::N9,

        Equal | NumpadEqual => Key::Equal,

        KeyA => Key::A,
        KeyB => Key::B,
        KeyC => Key::C,
        KeyD => Key::D,
        KeyE => Key::E,
        KeyF => Key::F,
        KeyG => Key::G,
        KeyH => Key::H,
        KeyI => Key::I,
        KeyJ => Key::J,
        KeyK => Key::K,
        KeyL => Key::L,
        KeyM => Key::M,
        KeyN => Key::N,
        KeyO => Key::O,
        KeyP => Key::P,
        KeyQ => Key::Q,
        KeyR => Key::R,
        KeyS => Key::S,
        KeyT => Key::T,
        KeyU => Key::U,
        KeyV => Key::V,
        KeyW => Key::W,
        KeyX => Key::X,
        KeyY => Key::Y,
        KeyZ => Key::Z,

        Minus | NumpadSubtract => Key::Minus,
        Period | NumpadDecimal => Key::Period,
        Quote => Key::Quote,
        Semicolon => Key::Semicolon,
        Slash | NumpadDivide => Key::Slash,

        // Skips Alts here

        Backspace | NumpadBackspace => Key::Backspace,
        // Skips Caps Lock
        ContextMenu => Key::Context,
        // Skips Controls here
        Enter | NumpadEnter => Key::Enter,
        // skips Super and Shift
        Space => Key::Space,
        Tab => Key::Tab,
        // Skips a few japanese exclusive keys here. Maybe ill add them later?
        Delete => Key::Delete,
        End => Key::End,
        Help => Key::Help,
        Home => Key::Home,
        Insert => Key::Insert,
        PageDown => Key::Pagedown,
        PageUp => Key::Pageup,
        ArrowDown => Key::Arrowdown,
        ArrowLeft => Key::Arrowleft,
        ArrowRight => Key::Arrowright,
        ArrowUp => Key::Arrowup,
        NumLock => Key::Numlock,

        // numlock keys are paired with the number keys above
        // all numpad keys that dont have a pairing above are ignored :)

        Escape => Key::Escape,
        // Fn, Fnlock, Prtsc skipped
        ScrollLock => Key::Scrolllock,

        // Pause, browser commands, media keys and system keys skipped
        F1 => Key::F1,
        F2 => Key::F2,
        F3 => Key::F3,
        F4 => Key::F4,
        F5 => Key::F5,
        F6 => Key::F6,
        F7 => Key::F7,
        F8 => Key::F8,
        F9 => Key::F9,
        F10 => Key::F10,
        F11 => Key::F11,
        F12 => Key::F12,

        ControlLeft | ControlRight => Key::Control,
        AltLeft | AltRight => Key::Alt,
        ShiftLeft | ShiftRight => Key::Shift,

        // last function keys skpped

        _ => {
            return None;
        }
    })
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash, Sequence)]
pub enum Key {
    Grave,
    Backslash,
    Bracketleft,
    Bracketright,
    Comma,
    Yen,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    Backspace,
    Context,
    Enter,
    Space,
    Tab,
    Delete,
    End,
    Help,
    Home,
    Insert,
    Pagedown,
    Pageup,
    Arrowdown,
    Arrowleft,
    Arrowright,
    Arrowup,
    Numlock,
    Escape,
    Scrolllock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Control,
    Shift,
    Alt,
    M1,
    M2,
    M3
}
