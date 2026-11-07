//! Standalone UI only. No world writes, input capture, or access to other mods.
use horizon_mod_sdk::{
    Mod, Registration, storage, input, command,
    ui::{self, ButtonWidget, Screen, TextWidget, Widget},
};

const LANGUAGE: u32 = 10;
const HELP: u32 = 11;
const ABOUT: u32 = 12;
const BACK: u32 = 13;
const CLOSE: u32 = 14;

#[derive(Clone, Copy, Default)]
enum Page {
    #[default]
    Home,
    Help,
    About,
    Macros,
}

#[derive(Default)]
struct Menu {
    english: bool,
    page: Page,
    screen_open: bool,
    notice: bool,
    shortcuts: Vec<Shortcut>,
}

#[derive(Clone)]
struct Shortcut { key: String, label: String, line: String }
impl Shortcut {
    fn parse(raw: &str) -> Result<Self,String> {
        let fields: Vec<_> = raw.split('|').map(str::trim).collect();
        if fields.len()!=3 || raw.chars().any(char::is_control) { return Err("Пример: /hmacro K | Время | time query daytime".into()); }
        let key=fields[0].to_ascii_lowercase();
        let line=fields[2].trim_start_matches('/');
        if key.is_empty() || !key.bytes().all(|b|b.is_ascii_alphanumeric() || b==b'.') || fields[1].is_empty() || fields[1].chars().count()>32 || line.is_empty() || line.len()>256 { return Err("Проверь клавишу, название и команду".into()); }
        Ok(Self {key,label:fields[1].into(),line:line.into()})
    }
    fn register(&self, slot: usize)->Result<(),String>{
        input::register_key(&format!("slot{slot}"),&self.label,&format!("key.keyboard.{}",self.key))
    }
}
fn text(id: u32, value: &str) -> Widget {
    Widget::Text(TextWidget { id, text: value.into(), color: 0 })
}
fn button(id: u32, value: &str) -> Widget {
    Widget::Button(ButtonWidget { id, label: value.into() })
}

impl Menu {
    fn label<'a>(&self, ru: &'a str, en: &'a str) -> &'a str {
        if self.english { en } else { ru }
    }

    fn widgets(&self) -> Vec<Widget> {
        let mut rows = match self.page {
            Page::Home => vec![
                text(1, "Horizon Menu 0.1.0"),
                text(2, self.label("Отдельное меню .hmod", "Standalone .hmod menu")),
                button(LANGUAGE, self.label("Язык: Русский", "Language: English")),
                button(15, self.label("Мои макросы", "My macros")),
                button(HELP, self.label("Где настройки других модов?", "Where are other mods' settings?")),
                button(ABOUT, self.label("О моде", "About")),
            ],
            Page::Macros => {
                let mut items=vec![text(21,"/hmacro K | Время | time query daytime"),text(22,"/hmacro del K")];
                for (i,m) in self.shortcuts.iter().enumerate(){items.push(button(30+i as u32,&format!("{} · {}",m.key.to_uppercase(),m.label)));}
                items.push(button(BACK,self.label("Назад","Back"))); items
            }
            Page::Help => vec![
                text(1, self.label("Закрой это меню кнопкой Done / Готово.", "Close this menu with Done.")),
                text(2, self.label("Открой Options > Mods в Horizon.", "Open Options > Mods in Horizon.")),
                text(3, self.label("Выбери мод и нажми Settings, если есть.", "Choose a mod and tap Settings, if available.")),
                text(4, self.label("API .hmod не даёт управлять другими модами.", "The .hmod API cannot manage other mods.")),
                button(BACK, self.label("Назад", "Back")),
            ],
            Page::About => vec![
                text(1, "Horizon Menu 0.1.0"),
                text(2, self.label("Автор: anonimmus136ik-hue", "Author: anonimmus136ik-hue")),
                text(3, self.label("Открыть: /hmodmenu", "Open: /hmodmenu")),
                text(4, self.label("Сохраняет только свой язык.", "Only saves its own language preference.")),
                button(BACK, self.label("Назад", "Back")),
            ],
        };
        if self.notice {
            rows.push(text(20, self.label("Не удалось сохранить язык.", "Could not save language.")));
        }
        // The host supplies Done for settings pages; never close another screen.
        if self.screen_open {
            rows.push(button(CLOSE, self.label("Закрыть", "Close")));
        }
        rows
    }

    fn refresh(&self) {
        let widgets = self.widgets();
        if self.screen_open {
            ui::open_screen(&Screen { title: "Horizon Menu".into(), widgets });
        } else {
            ui::set_settings(&widgets);
        }
    }
}

impl Mod for Menu {
    fn init() -> Result<(Self, Registration), String> {
        let mut menu = Self {
            english: storage::get("language").as_deref() == Some(b"en"),
            ..Self::default()
        };
        if let Some(data)=storage::get("shortcuts-v1"){
            for row in String::from_utf8_lossy(&data).lines().take(6){
                if let Ok(m)=Shortcut::parse(row){m.register(menu.shortcuts.len())?;menu.shortcuts.push(m);}
            }
        }
        ui::set_settings(&menu.widgets());
        Ok((menu, Registration::new().command("hmodmenu").command("hmacro").on_tick()))
    }

    fn command(&mut self, name: &str, args: &str) -> Result<Option<String>, String> {
        if name=="hmacro" {
            let mut next=self.shortcuts.clone();
            if let Some(key)=args.trim().strip_prefix("del "){
                let i=next.iter().position(|m|m.key.eq_ignore_ascii_case(key.trim())).ok_or("Нет такой кнопки")?;
                next.remove(i);
            } else {
                let m=Shortcut::parse(args)?;
                let i=next.iter().position(|old|old.key==m.key).unwrap_or(next.len());
                if i>=6 { return Err("Доступно 6 кнопок".into()); }
                m.register(i)?;
                if i==next.len(){next.push(m);}else{next[i]=m;}
            }
            let encoded=next.iter().map(|m|format!("{}|{}|{}",m.key,m.label,m.line)).collect::<Vec<_>>().join("\n");
            storage::set("shortcuts-v1",encoded.as_bytes())?;
            for (i,m) in next.iter().enumerate(){m.register(i)?;}
            self.shortcuts=next;
            self.refresh();
            return Ok(Some("Макросы сохранены".into()));
        }
        if name != "hmodmenu" {
            return Err("Unknown command".into());
        }
        if !args.trim().is_empty() {
            return Err("Usage: /hmodmenu".into());
        }
        self.page = Page::Home;
        self.screen_open = true;
        self.refresh();
        Ok(Some("Horizon Menu opened".into()))
    }

    fn tick(&mut self,_:u64){
        for event in input::poll(){if let input::Event::KeyDown(name)=event{
            if let Some(m)=name.strip_prefix("slot").and_then(|s|s.parse::<usize>().ok()).and_then(|i|self.shortcuts.get(i)){
                command::run(&m.line);
            }
        }}
    }
    fn ui_select(&mut self, id: u32, row: u32) {
        if row != 0 { return; }
        match (self.page, id) {
            (Page::Home, LANGUAGE) => {
                self.english = !self.english;
                let value: &[u8] = if self.english { b"en" } else { b"ru" };
                self.notice = storage::set("language", value).is_err();
            }
            (Page::Home,15)=>self.page=Page::Macros,
            (Page::Macros,30..=35)=>{if let Some(m)=self.shortcuts.get((id-30) as usize){command::run(&m.line);}return;}
            (Page::Home, HELP) => self.page = Page::Help,
            (Page::Home, ABOUT) => self.page = Page::About,
            (_, BACK) => self.page = Page::Home,
            (_, CLOSE) if self.screen_open => {
                ui::close_screen();
                self.screen_open = false;
                return;
            }
            _ => return,
        }
        self.refresh();
    }

    fn ui_settings_opened(&mut self) {
        self.screen_open = false;
        self.page = Page::Home;
        self.refresh();
    }

    fn ui_closed(&mut self) {
        self.screen_open = false;
        self.page = Page::Home;
    }
}

horizon_mod_sdk::export_mod!(Menu);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_is_only_offered_on_our_own_screen() {
        let mut menu = Menu::default();
        let has_close = |m: &Menu| m.widgets().iter().any(
            |w| matches!(w, Widget::Button(b) if b.id == CLOSE));
        assert!(!has_close(&menu));
        menu.screen_open = true;
        assert!(has_close(&menu));
        menu.ui_closed();
        assert!(!has_close(&menu));
    }

    #[test]
    fn help_explains_the_actual_api_limit_in_both_languages() {
        let mut menu = Menu { page: Page::Help, ..Menu::default() };
        let lines = |m: &Menu| m.widgets().into_iter().filter_map(|w| match w {
            Widget::Text(t) => Some(t.text), _ => None,
        }).collect::<Vec<_>>().join(" ");
        assert!(lines(&menu).contains("не даёт управлять"));
        menu.english = true;
        assert!(lines(&menu).contains("cannot manage other mods"));
    }
}
