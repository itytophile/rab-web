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

#[inline_props]
pub(crate) fn Home<'a>(
    cx: Scope,
    locale: Locale,
    set_wishes: &'a UseState<im_rc::Vector<(DisplaySkill, u8)>>,
    talismans: &'a im_rc::Vector<Talisman>,
    set_saved_builds: &'a UseState<im_rc::Vector<(String, Build)>>,
    storage: &'a Storage,
) -> Element {
    let wishes = set_wishes.get().as_ref();
    let all_skills: im_rc::HashSet<DisplaySkill> =
        Skill::ALL.iter().copied().map(DisplaySkill).collect();
    let available_skills: im_rc::HashSet<DisplaySkill> =
        all_skills.difference(wishes.iter().map(|(skill, _)| *skill).collect());
    let (builds, set_builds) = use_state(&cx, Vec::<Build>::new);
    let (gender, set_gender) = use_state(&cx, Gender::default);
    let (weapon_slots, set_weapon_slots) = use_state(&cx, || [0u8; 3]);

    let rows = wishes.iter().enumerate().map(|(index, (skill, amount))| {
        rsx! {
            div { class: "field has-addons",
                SkillRow {
                    set_skills: set_wishes,
                    index: index,
                    skill: *skill,
                    amount: *amount,
                    locale: *locale
                }
            }
        }
    });

    let toggle_gender = move |_| {
        if gender == &Gender::Female {
            set_gender(Gender::Male);
        } else {
            set_gender(Gender::Female);
        }
    };

    let icon_button = if gender == &Gender::Female {
        "fa-solid fa-venus"
    } else {
        "fa-solid fa-mars"
    };

    let search_is_disabled = wishes.is_empty();

    let search_builds = move |_| {
        set_builds(pre_selection_then_brute_force_search(
            wishes
                .iter()
                .map(|(skill, amount)| (skill.0, *amount))
                .collect::<Vec<(Skill, u8)>>()
                .as_slice(),
            AllArmorSlices {
                arms: &ARMS.iter().map(Into::into).collect::<Vec<Armor>>(),
                chests: &CHESTS.iter().map(Into::into).collect::<Vec<Armor>>(),
                helmets: &HELMETS.iter().map(Into::into).collect::<Vec<Armor>>(),
                legs: &LEGS.iter().map(Into::into).collect::<Vec<Armor>>(),
                talismans: &talismans.iter().map(Into::into).collect::<Vec<Armor>>(),
                waists: &WAISTS.iter().map(Into::into).collect::<Vec<Armor>>(),
            },
            *gender,
            *weapon_slots,
        ));
    };

    let build_views = builds.iter().map(|build| {
        rsx! {BuildView {
            b: build,
            locale: *locale,
            set_saved_builds: set_saved_builds,
            storage: storage
        }}
    });

    let weapon_slot_buttons = weapon_slots.iter().enumerate().map(|(index, value)| {
        rsx!(SlotButton {
            set_slots: set_weapon_slots
            value: *value
            index: index
        })
    });

    cx.render(rsx!(
        div { class: "columns",
            div { class: "column",
                div { class: "field is-grouped",
                    AddSkill {
                        options: available_skills,
                        set_skills: set_wishes,
                        locale: *locale
                    }
                    div { class: "control",
                        button { class: "button is-info", disabled: "{search_is_disabled}", onclick: search_builds,
                            span { class: "icon is-small",
                                i { class: "fa-solid fa-magnifying-glass" }
                            }
                            span { [UiSymbole::SearchBuilds.translate(*locale)] }
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
                        weapon_slot_buttons
                    }
                }
                rows
            }
            div { class: "column",
                build_views
            }
        }
    ))
}

#[inline_props] // can't use build as parameter name
fn BuildView<'a>(
    cx: Scope,
    b: &'a Build,
    locale: Locale,
    set_saved_builds: &'a UseState<im_rc::Vector<(String, Build)>>,
    storage: &'a Storage,
) -> Element {
    let (build_name, set_build_name) = use_state(&cx, String::new);
    let locale = *locale;
    let b = *b;

    let mut all_skills: Vec<(Skill, u8)> = b.get_all_skills_and_amounts().drain().collect();
    all_skills.sort_unstable_by_key(|(_, amount)| *amount);

    let placeholder = UiSymbole::BuildsName.translate(locale);

    let save_build = |_| {
        set_saved_builds.with_mut(|builds| {
            builds.push_back((build_name.clone(), b.clone()));
            storage.builds().save(builds);
        });
        set_build_name(String::new());
    };

    let save_disabled = build_name.is_empty();

    cx.render(rsx!(article { class: "panel is-info",
        p { class: "panel-heading" }
        div { class: "panel-block",
            div { class: "field has-addons is-expanded is-flex-grow-1",
                div { class: "control is-expanded",
                    input {
                        class: "input",
                        r#type: "text",
                        placeholder: "{placeholder}",
                        oninput: |event| set_build_name(event.value.clone())
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
                        span { [UiSymbole::Save.translate(locale)] }
                    }
                }
            }
        }
        BuildDetails { b: b, locale: locale }
    }))
}
