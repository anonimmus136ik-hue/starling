//! A small client-side chat-command macro. The key is registered through
//! Horizon's input capability; commands are executed with the local player's
//! own permissions, exactly as if the player had typed them.
use horizon_mod_sdk::{Mod, Registration, chat, command, input, storage};

const KEY_NAME: &str = "run_macro";
const DEFAULT_KEY: &str = "key.keyboard.caps_lock";
const DEFAULT_COMMAND: &str = "say Hello!";

struct ChatMacro {
    command: String,
}

impl ChatMacro {
    fn load() -> String {
        storage::get("command")
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| DEFAULT_COMMAND.to_owned())
    }

    fn save(&self) -> Result<(), String> {
        storage::set("command", self.command.as_bytes())
    }
}

impl Mod for ChatMacro {
    fn init() -> Result<(Self, Registration), String> {
        input::register_key(KEY_NAME, "Run chat macro", DEFAULT_KEY)
            .map_err(|error| format!("Could not register Caps Lock macro key: {error}"))?;
        let this = Self { command: Self::load() };
        chat::show(&format!("Ready. Macro: {}. Change with /chatmacro set <command>", this.command));
        Ok((this, Registration::new().command("chatmacro").on_tick()))
    }

    fn tick(&mut self, _tick: u64) {
        for event in input::poll() {
            if let input::Event::KeyDown(name) = event {
                if name == KEY_NAME {
                    command::run(&self.command);
                }
            }
        }
    }

    fn command(&mut self, _name: &str, args: &str) -> Result<Option<String>, String> {
        let args = args.trim();
        let Some(value) = args.strip_prefix("set ") else {
            return Ok(Some(format!("Current macro: {}. Usage: /chatmacro set <command>", self.command)));
        };
        let value = value.trim().trim_start_matches('/');
        if value.is_empty() {
            return Err("Usage: /chatmacro set <command>".into());
        }
        self.command = value.to_owned();
        self.save()?;
        Ok(Some(format!("Macro saved: {}", self.command)))
    }
}

horizon_mod_sdk::export_mod!(ChatMacro);
