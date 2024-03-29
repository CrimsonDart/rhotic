use std::{num::NonZeroU32, fmt::{Debug}};
use enum_iterator::{Sequence, all};
use softbuffer::{Context, Surface};
use winit::{
    window::{WindowBuilder},
    event_loop::EventLoop, event::{ElementState}, dpi::PhysicalSize, keyboard::{PhysicalKey, KeyCode}};

use crate::{state::application::State, buffer::stage::InputEvent};

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
                        state.send_event(InputEvent::Scroll(delta));
                        window.request_redraw();
                    },

                    MouseInput { device_id: _, state: pressed, button } => {

                        use winit::event::MouseButton::*;

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

                        if pressed == ElementState::Pressed {
                            state.send_event(InputEvent::Press(buf));
                        } else {
                            state.send_event(InputEvent::Release(buf));
                        }

                        state.input[buf] = pressed == ElementState::Pressed;

                        window.request_redraw();
                    },
                    KeyboardInput { device_id: _, event, is_synthetic: _ } => {

                        if event.state == ElementState::Pressed {
                            if let Some(_s) = event.logical_key.to_text() {
                                //TODO
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

                        let send_event = if event.state == ElementState::Pressed {
                            state.input[key] = true;
                            if event.repeat {
                                InputEvent::Echo(key)
                            } else {
                                InputEvent::Press(key)
                            }
                        } else {
                            state.input[key] = false;
                            InputEvent::Release(key)
                        };

                        window.request_redraw();

                        state.send_event(send_event);
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
    pub array: [bool; 85],
}

impl Input {
    pub fn get_pressed_keys(&self) -> Vec<Key> {
        let mut out = Vec::new();

        for i in all::<Key>() {
            if self.array[i as usize] {
                out.push(i);
            }
        }

        out
    }
}

impl Default for Input {
    fn default() -> Self {

        Self {
            mouse_position: Point::new(0, 0),
            array: [false; 85],
        }
    }
}

impl std::ops::Index<Key> for Input {
    type Output = bool;
    fn index(&self, index: Key) -> &Self::Output {
        &self.array[index as usize]
    }
}

impl std::ops::IndexMut<Key> for Input {
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        &mut self.array[index as usize]
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
