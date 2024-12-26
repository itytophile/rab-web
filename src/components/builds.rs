use crate::{
    components::BuildDetails,
    locale::{Locale, Translation, UiSymbole},
    storage::MyStorage,
    Route,
};
use dioxus::prelude::*;
use rab_core::{armor_and_skills::Skill, build_search::Build};
use web_sys::Storage;

#[inline_props]
pub(crate) fn Builds<'a>(
    cx: Scope,
    saved_builds: &'a UseState<im_rc::Vector<(String, Build)>>,
    storage: &'a Storage,
    locale: Locale,
    route: &'a UseState<Route>,
) -> Element<'a> {
    let locale = *locale;
    let builds = saved_builds
        .iter()
        .take(saved_builds.len() / 2)
        .enumerate()
        .map(|(index, (name, build))| {
            rsx!(BuildView {
                b: build,
                locale: locale,
                saved_builds: saved_builds,
                storage: storage,
                name: name,
                index: index
            })
        });
    let other_builds = saved_builds
        .iter()
        .skip(saved_builds.len() / 2)
        .enumerate()
        .map(|(index, (name, build))| {
            rsx!(BuildView {
                b: build,
                locale: locale,
                saved_builds: saved_builds,
                storage: storage,
                name: name,
                index: index
            })
        });
    cx.render(rsx!(if saved_builds.is_empty() {
        rsx!(
            [UiSymbole::NoSavedBuilds.translate(locale)]
            a {
                onclick: move |_| route.set(Route::Home),
                span { class: "icon is-small mr-1",
                    i { class: "fa-solid fa-magnifying-glass" }
                }
                span { [UiSymbole::Home.translate(locale)] }
            }
        )
    } else {
        rsx!(div { class: "columns",
            div { class: "column",
                other_builds
            }
            div { class: "column",
                builds
            }
        })
    }))
}

#[inline_props] // can't use build as parameter name
fn BuildView<'a>(
    cx: Scope,
    b: &'a Build,
    locale: Locale,
    saved_builds: &'a UseState<im_rc::Vector<(String, Build)>>,
    storage: &'a Storage,
    name: &'a str,
    index: usize,
) -> Element<'a> {
    let locale = *locale;
    let index = *index;
    let b = *b;

    let mut all_skills: Vec<(Skill, u8)> = b.get_all_skills_and_amounts().drain().collect();
    all_skills.sort_unstable_by_key(|(_, amount)| *amount);

    let delete_build = move |_| {
        saved_builds.with_mut(|saved_builds| {
            saved_builds.remove(index);
            storage.builds().save(saved_builds)
        })
    };

    cx.render(rsx!(article { class: "panel is-info",
        p { class: "message-header",
            "{name}"
            button {
                class: "delete",
                onclick: delete_build
            }
        }
        BuildDetails { b: b, locale: locale }
    }))
}
