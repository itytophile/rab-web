use crate::{
    components::BuildDetails,
    locale::{Locale, Translation, UiSymbole},
    storage::MyStorage,
    Route,
};
use dioxus::prelude::*;
use rab_core::{armor_and_skills::Skill, build_search::Build};
use web_sys::Storage;

#[component]
pub(crate) fn Builds(
    saved_builds: Signal<im_rc::Vector<(String, Build)>>,
    storage: ReadOnlySignal<Storage>,
    locale: Locale,
    route: Signal<Route>,
) -> Element {
    let saved_builds_r = saved_builds.read();
    let builds = saved_builds_r
        .iter()
        .enumerate()
        .take(saved_builds_r.len() / 2)
        .map(|(index, _)| {
            rsx!(BuildView {
                locale: locale,
                saved_builds: saved_builds,
                storage: storage,
                index: index
            })
        });
    let other_builds = saved_builds_r
        .iter()
        .enumerate()
        .skip(saved_builds_r.len() / 2)
        .map(|(index, _)| {
            rsx!(BuildView {
                locale: locale,
                saved_builds: saved_builds,
                storage: storage,
                index: index
            })
        });
    if saved_builds_r.is_empty() {
        rsx!(
            {UiSymbole::NoSavedBuilds.translate(locale)}
            a {
                onclick: move |_| route.set(Route::Home),
                span { class: "icon is-small mr-1",
                    i { class: "fa-solid fa-magnifying-glass" }
                }
                span { {UiSymbole::Home.translate(locale)} }
            }
        )
    } else {
        rsx!(div { class: "columns",
            div { class: "column",
                {other_builds}
            }
            div { class: "column",
                {builds}
            }
        })
    }
}

#[component] // can't use build as parameter name
fn BuildView(
    locale: Locale,
    saved_builds: Signal<im_rc::Vector<(String, Build)>>,
    storage: ReadOnlySignal<Storage>,
    index: usize,
) -> Element {
    let (name, build) = &saved_builds.read()[index];
    let mut all_skills: Vec<(Skill, u8)> = build.get_all_skills_and_amounts().drain().collect();
    all_skills.sort_unstable_by_key(|(_, amount)| *amount);

    let delete_build = move |_| {
        saved_builds.with_mut(|saved_builds| {
            saved_builds.remove(index);
            storage.read().builds().save(saved_builds)
        })
    };

    rsx!(article { class: "panel is-info",
        p { class: "message-header",
            "{name}"
            button {
                class: "delete",
                onclick: delete_build
            }
        }
        BuildDetails { b: build.clone(), locale: locale }
    })
}
