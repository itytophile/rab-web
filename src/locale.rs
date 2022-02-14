use self::{en::English, fr::French};

pub mod en;
pub mod fr;

#[derive(Clone, Copy)]
pub enum Locale {
    English,
    French,
}

pub enum UiSymbole {
    AddWish,
    SearchBuilds,
}

impl UiSymbole {
    pub fn translate(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::English => self.to_english(),
            Locale::French => self.to_french(),
        }
    }
}
