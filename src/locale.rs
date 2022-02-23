mod de;
pub mod en;
mod fr;

use self::{de::German, en::English, fr::French};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, strum_macros::EnumIter, Serialize, Deserialize)]
pub enum Locale {
    English,
    French,
    German,
}

impl Locale {
    pub fn native(&self) -> &'static str {
        match self {
            Locale::English => "English",
            Locale::French => "Français",
            Locale::German => "Deutsch",
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

pub trait Translation: English + French + German {
    fn translate(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::English => self.to_english(),
            Locale::French => self.to_french(),
            Locale::German => self.to_german(),
        }
    }
}

impl Translation for crate::DisplaySkill {}
impl Translation for crate::UiSymbole {}
impl Translation for crate::armors::Armor {}
