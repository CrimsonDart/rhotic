use std::{collections::HashMap, marker::PhantomData, time::Duration, error::Error, string::FromUtf8Error};

use winit::keyboard::KeyCode;

use toml::Table;

use super::stage::{Function, Stage};

pub type Func<S: Stage> = fn(&mut S) -> bool;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Command {
    bites: Vec<Bite>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Bite {
    shift: bool,
    alt: bool,
    control: bool,
    key: &'static str
}

pub struct FunctionMap<S: Stage> {
    map: HashMap<&'static str, Func<S>>
}

impl<S: Stage + 'static> FunctionMap<S> {
    pub fn new() -> Self {

        let mut map = HashMap::new();

        for (key, value) in S::get_functions() {
            map.insert(*key, *value);
        }
        Self {
            map
        }
    }

    pub fn bind(self, table: toml::Table) -> CommandBinds<S> {

        use toml::Value::*;

        let keybinds = if let Some(Table(keybinds))= table.get("keybinds") {
            keybinds
        } else {
            return CommandBinds { command_map: HashMap::new(), name_map: self.map };
        };

        let mut command_map = HashMap::new();

        for (name, func) in self.map.iter() {

            let bind = if let Some(s) = keybinds.get(*name) {
                s
            } else {
                continue;
            };

            match bind {
                String(string) => {
                    if let Some(c) = Command::from_string(string) {
                        command_map.insert(c, *func);
                    }
                },
                Array(array) => {
                   for value in array {
                       if let String(s) = value {
                           if let Some(c) = Command::from_string(s) {
                               command_map.insert(c, *func);
                           }
                       }
                   }

                },
                _ => { continue; }
            }
        }

        CommandBinds { command_map, name_map: self.map }

    }
}

#[derive(Debug)]
pub struct CommandBinds<S: Stage> {
    command_map: HashMap<Command, Func<S>>,
    name_map: HashMap<&'static str, Func<S>>,
}

pub enum FunctionCallError {
    FunctionFail,
    FunctionNotFound
}

impl<S: Stage> CommandBinds<S> {

    pub fn name_call(&self, stage: &mut S, string: &str) -> Result<(), FunctionCallError> {
        if let Some(value) = self.name_map.get(string) {
            if value(stage) {
                Ok(())
            } else {
                Err(FunctionCallError::FunctionFail)
            }
        } else {
            Err(FunctionCallError::FunctionNotFound)
        }
    }

    pub fn command_call(&self, stage: &mut S, command: &Command) -> Result<(), FunctionCallError> {
        if let Some(value) = self.command_map.get(command) {
            if value(stage) {
                Ok(())
            } else {
                Err(FunctionCallError::FunctionFail)
            }
        } else {
            Err(FunctionCallError::FunctionNotFound)
        }
    }
}




impl Command {
    pub fn to_string(&self) -> String {

        let mut out = String::new();
        let mut is_first = true;

        for bite in self.bites.iter() {
            if !is_first {
                out.push(' ');
            } else {
                is_first = false;
            }
            out.push_str(bite.to_string().as_str());
        }

        out
    }

    pub fn from_string(s: &str) -> Option<Self> {
        if !s.is_ascii() {
            return None;
        }

        let mut out = Vec::new();

        for sting in s.split(' ') {
            if let Some(b) = Bite::from_string(sting) {
                out.push(b);
            } else {
                return None;
            }
        }

        if out.is_empty() {
            return None;
        }

        Some(Self {
            bites: out
        })
    }

    pub fn push(&mut self, key: KeyCode, shift: bool, control: bool, alt: bool) -> bool {
        let key = get_keycode_name(key);

        let key = if let Some(k) = key {
            k
        } else {
            return false;
        };

        self.bites.push(
            Bite { shift, alt, control, key }
        );

        true
    }
}

impl Bite {
    fn to_string(&self) -> String {
        let mut out = String::new();

        let mut is_mod = false;

        if self.shift {
            out.push('S');
            is_mod = true;
        }

        if self.control {
            out.push('C');
            is_mod = true;
        }

        if self.alt {
            out.push('A');
            is_mod = true;
        }

        if is_mod {
            out.push('-');
        }

        out.push_str(self.key);

        out
    }

    fn from_string(s: &str) -> Option<Self> {
        let mut chars = s.chars();

        let mut out = Self {
            shift: false,
            control: false,
            alt: false,
            key: ""
        };

        for _ in 0..4 {
            match chars.next() {
                Some('S') => out.shift = true,
                Some('C') => out.control = true,
                Some('A') => out.alt = true,
                Some('-') => {
                    let test: String = chars.collect();

                    let mut success = false;

                    for e in VALID_KEYS {
                        if e == test {
                            out.key = e;
                            success = true;
                        }
                    }

                    if !success {
                        return None;
                    }

                    break;
                },
                Some(_) => {
                    let mut success = false;

                    for e in VALID_KEYS {
                        if e == s {
                            out.key = e;
                            success = true;
                        }
                    }

                    if !success {
                        return None;
                    }

                    break;
                },
                None => {
                    return None;
                }
            }
        }

        Some(out)
    }
}

// Some keys are omitted for simplicity; This is to allow for portability. Some keyboards, for example, on a laptop,
// may not have a numberpad. so having keybinds that can only be used on a PC, or compatiable keyboard (i think) is a bad idea.
// I'm not against you having special keybinds to those keys, but
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


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn back_and_forth_conversion() {
        let CCC = "S-space";
        assert_eq!(Bite::from_string(CCC).unwrap().to_string(), "S-space");

        assert_ne!(Bite::from_string("AS-a").unwrap().to_string(), "AS-a");
    }
}
