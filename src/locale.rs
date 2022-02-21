pub mod en;
mod fr;

use self::{en::English, fr::French};

#[derive(Clone, Copy, PartialEq, strum_macros::EnumIter)]
pub enum Locale {
    English,
    French,
}

impl Locale {
    pub fn native(&self) -> &'static str {
        match self {
            Locale::English => "English",
            Locale::French => "Français",
        }
    }
}

pub enum UiSymbole {
    AddSkill,
    SearchBuilds,
    Filter,
    MyTalismans,
    Home,
    TalismansName,
    NoSlots,
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
impl Translation for crate::armors::Armor {}
