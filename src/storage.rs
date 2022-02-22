use crate::{locale::Locale, Talisman};
use rab_core::build_search::Build;

pub(crate) trait MyStorage {
    fn save_talismans(&self, talismans: &im_rc::Vector<Talisman>);
    fn get_talismans(&self) -> Option<im_rc::Vector<Talisman>>;
    fn save_locale(&self, locale: Locale);
    fn get_locale(&self) -> Option<Locale>;
    fn save_builds(&self, builds: &im_rc::Vector<(String, Build)>);
    fn get_builds(&self) -> Option<im_rc::Vector<(String, Build)>>;
}

const TALISMANS: &str = "talismans";
const LOCALE: &str = "locale";
const BUILDS: &str = "builds";

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

    fn save_builds(&self, builds: &im_rc::Vector<(String, Build)>) {
        self.set_item(BUILDS, &ron::to_string(&builds).unwrap())
            .unwrap()
    }

    fn get_builds(&self) -> Option<im_rc::Vector<(String, Build)>> {
        ron::from_str(&self.get_item(BUILDS).unwrap()?).ok()
    }
}
