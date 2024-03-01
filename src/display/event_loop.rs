use std::{num::NonZeroU32, fmt::{Display, Debug}, time::Instant, collections::HashMap};
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

                        let mut buffer = surface.buffer_mut().unwrap();
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


                        if event.state == ElementState::Pressed {
                            if let Some(s) = event.logical_key.to_text() {
                                state.input.text.push_str(s);
                            }

                        }

                        if let PhysicalKey::Code(code) = event.physical_key {
                            if event.state == ElementState::Pressed {
                                if event.repeat {
                                    state.input.keys.entry(code).and_modify(|x| {
                                        if let ButtonState::Held(t) = *x {
                                            *x = ButtonState::Echo(t)
                                        }
                                    })
                                    .or_insert(ButtonState::Pressed);
                                    return;
                                }
                                state.input.keys.insert(code, ButtonState::Pressed);
                                return;
                            }

                            state.input.keys.entry(code).or_insert(ButtonState::Released);
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

    pub keys: HashMap<KeyCode, ButtonState>,
    pub text: String,
}

impl Input {
    pub fn advance_state(&mut self) {

        self.scroll_delta = None;

        self.m1 = self.m1.advance_state();
        self.m2 = self.m2.advance_state();
        self.m3 = self.m3.advance_state();


        self.keys = self.keys.iter_mut()
            .filter_map(|(key, value)| {
                match value {
                    ButtonState::Depressed | ButtonState::Released => None,
                    ButtonState::Pressed => Some((*key, ButtonState::Held(Instant::now()))),
                    ButtonState::Held(t) | ButtonState::Echo(t) => Some((*key, ButtonState::Held(*t)))
                }
            })
            .collect();

        self.text = String::new();
    }

    pub fn get_pressed_keys(&self) -> Vec<KeyCode> {
        let mut out = Vec::new();

        for (k, b) in self.keys.iter() {
            if b.is_pressed() {
                out.push(k.clone());
            }
        }

        out
    }

    pub fn is_key_pressed(&self, key: &KeyCode) -> bool {

        if let Some(button) = self.keys.get(key) {
            use ButtonState::*;
            match button {
                Pressed | Echo(_) | Held(_) => true,
                _ => false
            }
        } else {
            false
        }
    }

    pub fn is_any_key_pressed(&self, keys: &[KeyCode]) -> bool {
        for key in keys {
            if self.is_key_pressed(key) {
                return true;
            } else {
                continue;
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
            m1: Depressed,
            m2: Depressed,
            m3: Depressed,

            keys: HashMap::new(),
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

    pub fn is_pressed(&self) -> bool {
        use ButtonState::*;
        match self {
            Pressed | Echo(_) | Held(_) => true,
            _ => false
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


const fn get_keycode_name(key: KeyCode) -> Option<&'static str> {
    use KeyCode::*;
    Some(match key {
        Backquote => "grave",
        Backslash | IntlRo => "\\",
        BracketLeft => "[",
        BracketRight | IntlBackslash => "]",
        Comma | NumpadComma => ",",
        IntlYen => "¥",

        Digit0 | Numpad0 => "0",
        Digit1 | Numpad1 => "1",
        Digit2 | Numpad2 => "2",
        Digit3 | Numpad3 => "3",
        Digit4 | Numpad4 => "4",
        Digit5 | Numpad5 => "5",
        Digit6 | Numpad6 => "6",
        Digit7 | Numpad7 => "7",
        Digit8 | Numpad8 => "8",
        Digit9 | Numpad9 => "9",

        Equal | NumpadEqual => "=",

        KeyA => "a",
        KeyB => "b",
        KeyC => "c",
        KeyD => "d",
        KeyE => "e",
        KeyF => "f",
        KeyG => "g",
        KeyH => "h",
        KeyI => "i",
        KeyJ => "j",
        KeyK => "k",
        KeyL => "l",
        KeyM => "m",
        KeyN => "n",
        KeyO => "o",
        KeyP => "p",
        KeyQ => "q",
        KeyR => "r",
        KeyS => "s",
        KeyT => "t",
        KeyU => "u",
        KeyV => "v",
        KeyW => "w",
        KeyX => "x",
        KeyY => "y",
        KeyZ => "z",

        Minus | NumpadSubtract => "-",
        Period | NumpadDecimal => ".",
        Quote => "\"",
        Semicolon => ";",
        Slash | NumpadDivide => "/",

        // Skips Alts here

        Backspace | NumpadBackspace => "backspace",
        // Skips Caps Lock
        ContextMenu => "menu",
        // Skips Controls here
        Enter | NumpadEnter => "enter",
        // skips Super and Shift
        Space => "space",
        Tab => "tab",
        // Skips a few japanese exclusive keys here. Maybe ill add them later?
        Delete => "delete",
        End => "end",
        Help => "help",
        Home => "home",
        Insert => "insert",
        PageDown => "pagedown",
        PageUp => "pageup",
        ArrowDown => "arrowdown",
        ArrowLeft => "arrowleft",
        ArrowRight => "arrowright",
        ArrowUp => "arrowup",
        NumLock => "numlock",

        // numlock keys are paired with the number keys above
        // all numpad keys that dont have a pairing above are ignored :)

        Escape => "escape",
        // Fn, Fnlock, Prtsc skipped
        ScrollLock => "scrolllock",

        // Pause, browser commands, media keys and system keys skipped
        F1 => "f1",
        F2 => "f2",
        F3 => "f3",
        F4 => "f4",
        F5 => "f5",
        F6 => "f6",
        F7 => "f7",
        F8 => "f8",
        F9 => "f9",
        F10 => "f10",
        F11 => "f11",
        F12 => "f12",

        // last function keys skpped

        _ => {
            return None;
        }
    })
}

const VALID_KEYS: [&'static str; 77] = [
    "grave",
    "\\",
    "[",
    "]",
    ",",
    "¥",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",

    "=",

    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",

    "-",
    ".",
    "\"",
    ";",
    "/",

    "backspace",
    "menu",
    "enter",
    "space",
    "tab",
    "delete",
    "end",
    "help",
    "home",
    "insert",
    "pagedown",
    "arrowleft",
    "arrowright",
    "arrowup",
    "numlock",
    "escape",
    "scrolllock",
    "f1",
    "f2",
    "f3",
    "f4",
    "f5",
    "f6",
    "f7",
    "f8",
    "f9",
    "f10",
    "f11",
    "f12",
];

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
    Alt
}
