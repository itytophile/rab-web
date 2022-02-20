#![allow(non_snake_case)]

mod armors;
mod components;
mod locale;

use crate::locale::UiSymbole;
use components::Home;
use dioxus::prelude::*;
use locale::{Locale, Translation};
use rab_core::armor_and_skills::Skill;
use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DisplaySkill(Skill);

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

    let routes = match route {
        Route::Home => cx.render(rsx!(Home { locale: *locale })),
        Route::Talismans => cx.render(rsx!(Talismans {})),
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

fn Talismans(cx: Scope) -> Element {
    cx.render(rsx!("lol"))
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
                                a { class: "button", onclick: move |_| set_route(Route::Home),
                                    span { class: "icon is-small",
                                        i { class: "fa-solid fa-house" }
                                    }
                                    span { [UiSymbole::Home.translate(locale)] }
                                }
                                a { class: "button", onclick: move |_| set_route(Route::Talismans),
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
