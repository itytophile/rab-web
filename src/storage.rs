use std::marker::PhantomData;

use crate::{locale::Locale, Talisman};
use rab_core::build_search::Build;
use serde::{de::DeserializeOwned, Serialize};
use web_sys::Storage;

pub(crate) trait MyStorage {
    fn talismans(&self) -> Resource<im_rc::Vector<Talisman>>;
    fn locale(&self) -> Resource<Locale>;
    fn builds(&self) -> Resource<im_rc::Vector<(String, Build)>>;
}

const TALISMANS: &str = "talismans";
const LOCALE: &str = "locale";
const BUILDS: &str = "builds";

impl MyStorage for Storage {
    fn talismans(&self) -> Resource<im_rc::Vector<Talisman>> {
        Resource::new(TALISMANS, self)
    }

    fn locale(&self) -> Resource<Locale> {
        Resource::new(LOCALE, self)
    }

    fn builds(&self) -> Resource<im_rc::Vector<(String, Build)>> {
        Resource::new(BUILDS, self)
    }
}

pub(crate) struct Resource<'a, T: Serialize + DeserializeOwned>(
    &'static str,
    &'a Storage,
    PhantomData<T>,
);

impl<'a, T: Serialize + DeserializeOwned> Resource<'a, T> {
    pub(crate) fn new(key: &'static str, storage: &'a Storage) -> Resource<'a, T> {
        Resource(key, storage, PhantomData)
    }

    pub(crate) fn save(self, value: &T) {
        let Resource(key, storage, _) = self;
        storage
            .set_item(key, &ron::to_string(value).unwrap())
            .unwrap()
    }

    pub(crate) fn get(self) -> Option<T> {
        let Resource(key, storage, _) = self;
        ron::from_str(&storage.get_item(key).unwrap()?).ok()
    }
}
