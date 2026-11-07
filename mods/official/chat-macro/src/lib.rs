//! Two configurable key macros for the Android client.
//! Their physical keys are rebound in Options > Mods by the game's own
//! key-binding UI. The commands run with the local player's permissions.
use horizon_mod_sdk::{
    Mod, Registration, chat, command, input, storage,
    ui::{self, ButtonWidget, Widget},
};

const KEYS: [(&str, &str, &str); 2] = [
    ("macro_1", "Run macro 1", "key.keyboard.right.control"),
    ("macro_2", "Run macro 2", "key.keyboard.right.shift"),
];
const BUTTONS: [u32; 2] = [1, 2];

struct ChatMacro {
    commands: [String; 2],
    enabled: [bool; 2],
}

impl ChatMacro {
    fn load_text(key: &str) -> String {
        storage::get(key)
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .unwrap_or_default()
            .trim()
            .to_owned()
    }

    fn load_bool(key: &str) -> bool {
        storage::get(key)
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .is_none_or(|value| value != b"false")
    }

    fn save(&self) -> Result<(), String> {
        for i in 0..2 {
            storage::set(&format!("command_{}", i + 1), self.commands[i].as_bytes())?;
            storage::set(
                &format!("enabled_{}", i + 1),
                if self.enabled[i] { b"true" } else { b"false" },
            )?;
        }
        Ok(())
    }

    fn page(&self) {
        ui::set_settings(&[
            Widget::Button(ButtonWidget {
                id: BUTTONS[0],
                label: format!("Macro 1: {}", if self.enabled[0] { "ON" } else { "OFF" }),
            }),
            Widget::Button(ButtonWidget {
                id: BUTTONS[1],
                label: format!("Macro 2: {}", if self.enabled[1] { "ON" } else { "OFF" }),
            }),
        ]);
    }

    fn run(&self, index: usize) {
        if self.enabled[index] && !self.commands[index].is_empty() {
            command::run(&self.commands[index]);
        }
    }
}

impl Mod for ChatMacro {
    fn init() -> Result<(Self, Registration), String> {
        for (name, label, default_key) in KEYS {
            input::register_key(name, label, default_key)
                .map_err(|error| format!("Could not register {label}: {error}"))?;
        }

        let this = Self {
            commands: [
                Self::load_text("command_1"),
                Self::load_text("command_2"),
            ],
            enabled: [
                Self::load_bool("enabled_1"),
                Self::load_bool("enabled_2"),
            ],
        };
        this.page();
        chat::show(
            "Chat Macro ready. Rebind keys in Options > Mods; use /chatmacro help for commands.",
        );
        Ok((this, Registration::new().command("chatmacro").on_tick()))
    }

    fn tick(&mut self, _tick: u64) {
        for event in input::poll() {
            if let input::Event::KeyDown(name) = event {
                match name.as_str() {
                    "macro_1" => self.run(0),
                    "macro_2" => self.run(1),
                    _ => {}
                }
            }
        }
    }

    fn command(&mut self, _name: &str, args: &str) -> Result<Option<String>, String> {
        let args = args.trim();
        if args == "help" || args.is_empty() {
            return Ok(Some(
                "Usage: /chatmacro set 1 <command> | set 2 <command> | clear 1|2 | on 1|2 | off 1|2. Rebind both keys in Options > Mods.".into(),
            ));
        }

        let (action, rest) = args.split_once(' ').unwrap_or((args, ""));
        let (slot, value) = if action == "set" {
            let (slot, value) = rest.trim().split_once(' ').ok_or("Usage: /chatmacro set 1|2 <command>")?;
            (slot, Some(value.trim().trim_start_matches('/')))
        } else {
            (rest.trim(), None)
        };
        let index = match slot {
            "1" => 0,
            "2" => 1,
            _ => return Err("Choose macro 1 or 2.".into()),
        };

        match action {
            "set" => {
                let value = value.unwrap_or_default();
                if value.is_empty() {
                    return Err("The command cannot be empty.".into());
                }
                self.commands[index] = value.to_owned();
            }
            "clear" => self.commands[index].clear(),
            "on" => self.enabled[index] = true,
            "off" => self.enabled[index] = false,
            _ => return Err("Use set, clear, on, off or help.".into()),
        }

        self.save()?;
        self.page();
        Ok(Some(format!(
            "Macro {}: {}{}",
            index + 1,
            if self.enabled[index] { "ON" } else { "OFF" },
            if self.commands[index].is_empty() {
                String::new()
            } else {
                format!(" — {}", self.commands[index])
            }
        )))
    }

    fn ui_select(&mut self, id: u32, _row: u32) {
        match id {
            1 => self.enabled[0] = !self.enabled[0],
            2 => self.enabled[1] = !self.enabled[1],
            _ => return,
        }
        let _ = self.save();
        self.page();
    }

    fn ui_settings_opened(&mut self) {
        self.page();
    }
}

horizon_mod_sdk::export_mod!(ChatMacro);
