#![allow(non_snake_case)]

mod armors;
mod components;
mod locale;
mod storage;

use components::{Builds, Home, Talismans};
use dioxus::prelude::*;
use locale::{Locale, Translation, UiSymbole};
use rab_core::armor_and_skills::{Armor, Skill};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use storage::MyStorage;
use strum::IntoEnumIterator;
use web_sys::Storage;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DisplaySkill(Skill);

#[derive(Clone, Serialize, Deserialize)]
struct Talisman {
    name: String,
    skills: im_rc::Vector<(Skill, u8)>,
    slots: im_rc::Vector<u8>,
}

impl From<&Talisman> for Armor {
    fn from(
        Talisman {
            name,
            slots,
            skills,
        }: &Talisman,
    ) -> Self {
        Armor {
            name: name.clone(),
            skills: skills.iter().copied().collect(),
            slots: slots.iter().copied().collect(),
            rare: 1,
            defense: 0,
            fire: 0,
            water: 0,
            thunder: 0,
            ice: 0,
            dragon: 0,
            gender: None,
        }
    }
}

impl Deref for DisplaySkill {
    type Target = Skill;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Route {
    Home,
    Talismans,
    Builds,
}

fn app(cx: Scope) -> Element {
    let storage = use_state(&cx, || {
        web_sys::window().unwrap().local_storage().unwrap().unwrap()
    });
    let locale = use_state(&cx, || storage.locale().get().unwrap_or(Locale::English));
    let route = use_state(&cx, || Route::Home);
    let skills = use_state(&cx, im_rc::Vector::<(DisplaySkill, u8)>::new);
    let wishes = use_state(&cx, im_rc::Vector::<(DisplaySkill, u8)>::new);
    let talismans = use_state(&cx, || storage.talismans().get().unwrap_or_default());
    let saved_builds = use_state(&cx, || storage.builds().get().unwrap_or_default());

    let routes = match **route {
        Route::Home => rsx!(Home {
            locale: **locale,
            wishes: wishes,
            talismans: talismans,
            saved_builds: saved_builds,
            storage: storage
        }),
        Route::Talismans => rsx!(Talismans {
            locale: **locale,
            skills: skills,
            talismans: talismans,
            storage: storage
        }),
        Route::Builds => rsx!(Builds {
            saved_builds: saved_builds,
            storage: storage,
            locale: **locale,
            route: route
        }),
    };

    cx.render(rsx!(
    Navbar {
        locale: locale,
        route: route,
        storage: storage
    }
    section { class: "section",
        div { class: "container",
            routes
        }
    }))
}

#[inline_props]
fn Navbar<'a>(
    cx: Scope,
    locale: &'a UseState<Locale>,
    route: &'a UseState<Route>,
    storage: &'a Storage,
) -> Element<'a> {
    let is_dropdown_active = use_state(&cx, || false);
    let is_active = use_state(&cx, || false);

    let class_dropdown = if **is_dropdown_active {
        "dropdown is-active"
    } else {
        "dropdown"
    };

    let locales = Locale::iter().map(|loc| {
        cx.render(rsx!(a {
            class:"dropdown-item",
            onclick: move |_| {
                locale.set(loc);
                is_dropdown_active.set(false);
                storage.locale().save(&loc)
            },
            [loc.native()]
        }))
    });

    let spans = (0..3).map(|_| {
        rsx!(span {
            aria_hidden: "true"
        })
    });

    let (burger_class, menu_class) = if **is_active {
        ("navbar-burger is-active", "navbar-menu is-active")
    } else {
        ("navbar-burger", "navbar-menu")
    };

    let links = [
        (Route::Home, "fa-solid fa-magnifying-glass", UiSymbole::Home),
        (
            Route::Talismans,
            "fa-solid fa-lightbulb",
            UiSymbole::MyTalismans,
        ),
        (Route::Builds, "fa-solid fa-star", UiSymbole::MyBuilds),
    ]
    .map(|(to_route, icon, label)| {
        let class = if to_route == ***route {
            "button is-static"
        } else {
            "button"
        };
        rsx!(a {
            class: "{class}",
            onclick: move |_| route.set(to_route),
            span { class: "icon is-small",
                i { class: "{icon}" }
            }
            span { [label.translate(***locale)] }
        })
    });

    cx.render(
        rsx!(nav { class: "navbar", role: "navigation", aria_label: "main navigation",
            div { class: "navbar-brand",
                div { class: "navbar-item",
                    div { class: "{class_dropdown}",
                        div { class: "dropdown-trigger",
                            a { class: "button", onclick: |_| *is_dropdown_active.make_mut() ^= true,
                                span { class: "icon is-small",
                                    i { class: "fa-solid fa-globe" }
                                }
                            }
                        }
                        div { class: "dropdown-menu",
                            div { class: "dropdown-content",
                                locales
                            }
                        }
                    }
                }
                a {
                    role: "button",
                    class: "{burger_class}",
                    onclick: |_| *is_active.make_mut() ^= true,
                    spans
                }
            }
            div { class: "{menu_class}",
                div { class: "navbar-end",
                    div { class: "navbar-item",
                        div { class:"buttons",
                            links
                        }
                    }
                }
            }
        }),
    )
}

fn main() {
    dioxus::web::launch(app);
}
