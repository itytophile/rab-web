use crate::{
    armors::{ARMS, CHESTS, HELMETS, LEGS, WAISTS},
    components::{AddSkill, BuildDetails, SkillRow, SlotButton},
    locale::{Locale, Translation, UiSymbole},
    storage::MyStorage,
    DisplaySkill, Talisman,
};
use dioxus::prelude::*;
use rab_core::{
    armor_and_skills::{Armor, Gender, Skill},
    build_search::{pre_selection_then_brute_force_search, AllArmorSlices, Build},
};
use web_sys::Storage;

#[component]
pub(crate) fn Home(
    locale: Locale,
    wishes: Signal<im_rc::Vector<(DisplaySkill, u8)>>,
    talismans: ReadOnlySignal<im_rc::Vector<Talisman>>,
    saved_builds: Signal<im_rc::Vector<(String, Build)>>,
    storage: ReadOnlySignal<Storage>,
) -> Element {
    let all_skills: im_rc::HashSet<DisplaySkill> =
        Skill::ALL.iter().copied().map(DisplaySkill).collect();
    let available_skills: im_rc::HashSet<DisplaySkill> =
        all_skills.difference(wishes.read().iter().map(|(skill, _)| *skill).collect());
    let mut builds = use_signal(Vec::<Build>::new);
    let mut gender = use_signal(Gender::default);
    let weapon_slots = use_signal(|| [0u8; 3]);

    let wishes_r = wishes.read();

    let rows = wishes_r.iter().enumerate().map(|(index, (skill, amount))| {
        rsx! {
            div { class: "field has-addons",
                SkillRow {
                    skills: wishes,
                    index: index,
                    skill: *skill,
                    amount: *amount,
                    locale: locale
                }
            }
        }
    });

    let toggle_gender = move |_| {
        gender.set(if *gender.read() == Gender::Female {
            Gender::Male
        } else {
            Gender::Female
        })
    };

    let icon_button = if *gender.read() == Gender::Female {
        "fa-solid fa-venus"
    } else {
        "fa-solid fa-mars"
    };

    let search_is_disabled = wishes.read().is_empty();

    let search_builds = move |_| {
        builds.set(pre_selection_then_brute_force_search(
            wishes
                .read()
                .iter()
                .map(|(skill, amount)| (skill.0, *amount))
                .collect::<Vec<(Skill, u8)>>()
                .as_slice(),
            AllArmorSlices {
                arms: &ARMS.iter().map(Into::into).collect::<Vec<Armor>>(),
                chests: &CHESTS.iter().map(Into::into).collect::<Vec<Armor>>(),
                helmets: &HELMETS.iter().map(Into::into).collect::<Vec<Armor>>(),
                legs: &LEGS.iter().map(Into::into).collect::<Vec<Armor>>(),
                talismans: &talismans
                    .read()
                    .iter()
                    .map(Into::into)
                    .collect::<Vec<Armor>>(),
                waists: &WAISTS.iter().map(Into::into).collect::<Vec<Armor>>(),
            },
            *gender.read(),
            *weapon_slots.read(),
        ));
    };

    let builds_r = builds.read();

    let build_views = builds_r.iter().map(|build| {
        rsx! {BuildView {
            b: build.clone(),
            locale: locale,
            saved_builds: saved_builds,
            storage: storage
        }}
    });

    let weapon_slots_r = weapon_slots.read();

    let weapon_slot_buttons = weapon_slots_r.iter().enumerate().map(|(index, value)| {
        rsx!(SlotButton {
            slots: weapon_slots,
            value: *value,
            index: index
        })
    });

    rsx!(
        div { class: "columns",
            div { class: "column",
                div { class: "field is-grouped",
                    AddSkill {
                        options: available_skills,
                        skills: wishes,
                        locale: locale
                    }
                    div { class: "control",
                        button { class: "button is-info", disabled: "{search_is_disabled}", onclick: search_builds,
                            span { class: "icon is-small",
                                i { class: "fa-solid fa-magnifying-glass" }
                            }
                            span { {UiSymbole::SearchBuilds.translate(locale)} }
                        }
                    }
                }
                div { class: "field is-grouped",
                    div { class: "control",
                        button { class: "button", onclick: toggle_gender,
                            span { class: "icon is-small",
                                i { class: "{icon_button}" }
                            }
                        }
                    }
                    div { class: "field has-addons",
                        div { class: "control",
                            button { class: "button is-static", onclick: toggle_gender,
                                span { class: "icon is-small",
                                    i { class: "fa-solid fa-hammer" }
                                }
                            }
                        }
                        {weapon_slot_buttons}
                    }
                }
                {rows}
            }
            div { class: "column",
                {build_views}
            }
        }
    )
}

#[component] // can't use build as parameter name
fn BuildView(
    b: ReadOnlySignal<Build>,
    locale: Locale,
    saved_builds: Signal<im_rc::Vector<(String, Build)>>,
    storage: ReadOnlySignal<Storage>,
) -> Element {
    let mut build_name = use_signal(String::new);

    let mut all_skills: Vec<(Skill, u8)> = b.read().get_all_skills_and_amounts().drain().collect();
    all_skills.sort_unstable_by_key(|(_, amount)| *amount);

    let placeholder = UiSymbole::BuildsName.translate(locale);

    let save_build = move |_| {
        saved_builds.with_mut(|builds| {
            builds.push_back((build_name.to_string(), b.read().clone()));
            storage.read().builds().save(builds);
        });
        build_name.set(String::new());
    };

    let save_disabled = build_name.read().is_empty();

    rsx!(article { class: "panel is-info",
        p { class: "panel-heading" }
        div { class: "panel-block",
            div { class: "field has-addons is-expanded is-flex-grow-1",
                div { class: "control is-expanded",
                    input {
                        class: "input",
                        r#type: "text",
                        placeholder: "{placeholder}",
                        oninput: move |event| build_name.set(event.value().clone())
                    }
                }
                div { class: "control",
                    button {
                        class: "button is-info",
                        onclick: save_build,
                        disabled: "{save_disabled}",
                        span { class: "icon is-small",
                            i { class: "fa-solid fa-star" }
                        }
                        span { {UiSymbole::Save.translate(locale)} }
                    }
                }
            }
        }
        BuildDetails { b: b, locale: locale }
    })
}
