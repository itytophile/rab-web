use crate::{locale::Locale, Talisman};

pub(crate) trait MyStorage {
    fn save_talismans(&self, talismans: &im_rc::Vector<Talisman>);
    fn get_talismans(&self) -> Option<im_rc::Vector<Talisman>>;
    fn save_locale(&self, locale: Locale);
    fn get_locale(&self) -> Option<Locale>;
}

const TALISMANS: &str = "talismans";
const LOCALE: &str = "locale";

impl MyStorage for web_sys::Storage {
    fn save_talismans(&self, talismans: &im_rc::Vector<Talisman>) {
        self.set_item(TALISMANS, &ron::to_string(talismans).unwrap())
            .unwrap()
    }

    fn get_talismans(&self) -> Option<im_rc::Vector<Talisman>> {
        ron::from_str(&self.get_item(TALISMANS).unwrap()?).ok()
    }

    fn save_locale(&self, locale: Locale) {
        self.set_item(LOCALE, &ron::to_string(&locale).unwrap())
            .unwrap()
    }

    fn get_locale(&self) -> Option<Locale> {
        ron::from_str(&self.get_item(LOCALE).unwrap()?).ok()
    }
}
