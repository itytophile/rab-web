mod en;
mod fr;

use self::{en::English, fr::French};

#[derive(Clone, Copy)]
pub enum Locale {
    English,
    French,
}

impl Locale {
    pub const ALL: [Locale; 2] = [Locale::English, Locale::French];

    pub fn native(&self) -> &'static str {
        match self {
            Locale::English => "English",
            Locale::French => "Français",
        }
    }
}

pub enum UiSymbole {
    AddWish,
    SearchBuilds,
}

pub trait Translation: English + French {
    fn translate(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::English => self.to_english(),
            Locale::French => self.to_french(),
        }
    }
}

impl Translation for crate::DisplaySkill {}
impl Translation for crate::UiSymbole {}
