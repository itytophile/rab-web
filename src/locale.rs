mod de;
pub mod en;
mod fr;
mod it;
mod pl;
mod ru;

use self::{de::German, en::English, fr::French, it::Italian, pl::Polish, ru::Russian};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, strum_macros::EnumIter, Serialize, Deserialize)]
pub enum Locale {
    English,
    French,
    German,
    Italian,
    Polish,
    Russian,
}

impl Locale {
    pub fn native(&self) -> &'static str {
        match self {
            Locale::English => "English",
            Locale::French => "Français",
            Locale::German => "Deutsch",
            Locale::Italian => "Italiano",
            Locale::Polish => "Polski",
            Locale::Russian => "Русский",
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
    ShowSkills,
    HideSkills,
    MyBuilds,
    Save,
    BuildsName,
    NoSavedBuilds,
    Jewel,
    Jewels,
}

pub trait Translation: English + French + German + Italian + Polish + Russian {
    fn translate(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::English => self.to_english(),
            Locale::French => self.to_french(),
            Locale::German => self.to_german(),
            Locale::Italian => self.to_italian(),
            Locale::Polish => self.to_polish(),
            Locale::Russian => self.to_russian(),
        }
    }
}

impl Translation for crate::DisplaySkill {}
impl Translation for crate::UiSymbole {}
impl Translation for crate::armors::Armor {}
