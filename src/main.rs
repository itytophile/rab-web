#![allow(non_snake_case)]

mod armors;
mod components;
mod locale;

use components::{Home, Talismans};
use dioxus::prelude::*;
use locale::{Locale, Translation, UiSymbole};
use rab_core::armor_and_skills::Skill;
use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DisplaySkill(Skill);

#[derive(Clone)]
struct Talisman {
    name: String,
    skills: im_rc::Vector<(Skill, u8)>,
    slots: im_rc::Vector<u8>,
}

impl Deref for DisplaySkill {
    type Target = Skill;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy)]
enum Route {
    Home,
    Talismans,
}

fn app(cx: Scope) -> Element {
    let (locale, set_locale) = use_state(&cx, || Locale::French);
    let (route, set_route) = use_state(&cx, || Route::Home);
    let (_, set_skills) = use_state(&cx, im_rc::Vector::<(DisplaySkill, u8)>::new);
    let (_, set_wishes) = use_state(&cx, im_rc::Vector::<(DisplaySkill, u8)>::new);
    let (_, set_talismans) = use_state(&cx, im_rc::Vector::<Talisman>::new);

    let locale = *locale;

    let routes = match route {
        Route::Home => rsx!(Home {
            locale: locale,
            set_wishes: set_wishes
        }),
        Route::Talismans => rsx!(Talismans {
            locale: locale,
            set_skills: set_skills,
            set_talismans: set_talismans
        }),
    };

    cx.render(rsx!(
    Navbar {
        set_locale: set_locale,
        set_route: set_route
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
    set_locale: &'a UseState<Locale>,
    set_route: &'a UseState<Route>,
) -> Element {
    let (is_active, set_is_active) = use_state(&cx, || false);

    let class_dropdown = if *is_active {
        "dropdown is-active"
    } else {
        "dropdown"
    };

    let locales = Locale::ALL.iter().map(|&locale| {
        cx.render(rsx!(a {
            class:"dropdown-item",
            onclick: move |_| {
                set_locale(locale);
                set_is_active(false);
            },
            [locale.native()]
        }))
    });

    let locale = **set_locale.get();
    let route = **set_route.get();

    let class_talisman = match route {
        Route::Talismans => "button is-primary",
        _ => "button",
    };

    let class_home = match route {
        Route::Home => "button is-primary",
        _ => "button",
    };

    cx.render(
        rsx!(nav { class: "navbar", role: "navigation", aria_label: "main navigation",
                div { class: "navbar-item",
                    div { class: "{class_dropdown}",
                        div { class: "dropdown-trigger",
                            div { class:"buttons",
                                a { class: "button", onclick: |_| set_is_active(!*is_active),
                                    span { class: "icon is-small",
                                        i { class: "fa-solid fa-globe" }
                                    }
                                }
                                a { class: "{class_home}", onclick: move |_| set_route(Route::Home),
                                    span { class: "icon is-small",
                                        i { class: "fa-solid fa-house" }
                                    }
                                    span { [UiSymbole::Home.translate(locale)] }
                                }
                                a { class: "{class_talisman}", onclick: move |_| set_route(Route::Talismans),
                                    span { class: "icon is-small",
                                        i { class: "fa-solid fa-lightbulb" }
                                    }
                                    span { [UiSymbole::MyTalismans.translate(locale)] }
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
        }),
    )
}

fn main() {
    dioxus::web::launch(app);
}
