use crate::armors::{self, ARMS, CHESTS, HELMETS, LEGS, WAISTS};
use crate::{
    locale::{en::English, Locale, Translation, UiSymbole},
    DisplaySkill,
};
use diacritics::remove_diacritics;
use dioxus::prelude::*;
use im_rc::{HashSet, Vector};
use lexical_sort::natural_lexical_cmp;
use rab_core::{
    armor_and_skills::{Armor, Gender, Skill},
    build_search::{pre_selection_then_brute_force_search, AllArmorSlices, Build},
};
use std::str::FromStr;

#[inline_props]
pub fn Home(cx: Scope, locale: Locale) -> Element {
    let (wishes, set_wishes) = use_state(&cx, Vector::<(DisplaySkill, u8)>::new);
    let all_skills: HashSet<DisplaySkill> = Skill::ALL.iter().copied().map(DisplaySkill).collect();
    let available_skills: HashSet<DisplaySkill> =
        all_skills.difference(wishes.iter().map(|(skill, _)| *skill).collect());
    let (builds, set_builds) = use_state(&cx, Vec::<Build>::new);
    let (gender, set_gender) = use_state(&cx, Gender::default);
    let (weapon_slots, set_weapon_slots) = use_state(&cx, || [0u8; 3]);

    let rows = wishes.iter().enumerate().map(|(index, (skill, amount))| {
        rsx! {
            div { class: "field has-addons",
                WishRow {
                    set_wishes: set_wishes,
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
                arms: &ARMS
                    .iter()
                    .map(|armor| armor.into())
                    .collect::<Vec<Armor>>(),
                chests: &CHESTS
                    .iter()
                    .map(|armor| armor.into())
                    .collect::<Vec<Armor>>(),
                helmets: &HELMETS
                    .iter()
                    .map(|armor| armor.into())
                    .collect::<Vec<Armor>>(),
                legs: &LEGS
                    .iter()
                    .map(|armor| armor.into())
                    .collect::<Vec<Armor>>(),
                talismans: &[],
                waists: &WAISTS
                    .iter()
                    .map(|armor| armor.into())
                    .collect::<Vec<Armor>>(),
            },
            *gender,
            *weapon_slots,
        ));
    };

    let build_views = builds.iter().map(|build| {
        rsx! {BuildView {
            b: build,
            locale: *locale
        }}
    });

    let weapon_slot_buttons = weapon_slots.iter().enumerate().map(|(index, value)| {
        rsx!(WeaponSlotButton {
            set_weapon_slots: set_weapon_slots
            value: *value
            index: index
        })
    });

    cx.render(rsx!(
        div { class: "columns",
            div { class: "column",
                div { class: "field is-grouped",
                    AddWish {
                        options: available_skills,
                        set_wishes: set_wishes,
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

#[inline_props]
fn WeaponSlotButton<'a>(
    cx: Scope,
    set_weapon_slots: &'a UseState<[u8; 3]>,
    value: u8,
    index: usize,
) -> Element {
    let index = *index;

    let increment = move |_| {
        set_weapon_slots.make_mut()[index] = (value + 1) % 4;
    };

    cx.render(rsx!(div { class: "control",
        button { class: "button", onclick: increment,
        span {class:"icon is-small", "{value}"}
        }
    }))
}

fn armor_to_string(armor: Option<&(Armor, [Option<Skill>; 3])>, locale: Locale) -> &str {
    if let Some((armor, _)) = armor {
        armors::Armor::from_str(&armor.name)
            .unwrap()
            .translate(locale)
    } else {
        "Free"
    }
}

#[inline_props] // can't use build a parameter name
fn BuildView<'a>(cx: Scope, b: &'a Build, locale: Locale) -> Element {
    let locale = *locale;

    cx.render(rsx!(article { class: "panel is-primary",
        p { class: "panel-heading" }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-hat-cowboy"}
            }
            [armor_to_string(b.helmet.as_ref(), locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-shirt"}
            }
            [armor_to_string(b.chest.as_ref(), locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-mitten"}
            }
            [armor_to_string(b.arm.as_ref(),locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-archway"}
            }
            [armor_to_string(b.waist.as_ref(),locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-socks"}
            }
            [armor_to_string(b.leg.as_ref(),locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-lightbulb"}
            }
            [armor_to_string(b.talisman.as_ref(),locale)]
        }
    }))
}

#[inline_props]
fn WishRow<'a>(
    cx: Scope,
    set_wishes: &'a UseState<Vector<(DisplaySkill, u8)>>,
    index: usize,
    skill: DisplaySkill,
    amount: u8,
    locale: Locale,
) -> Element {
    let (index, skill, amount) = (*index, *skill, *amount);

    let is_minus_disabled = amount == 1;
    let is_plus_disabled = amount == skill.get_limit();

    let remove_skill = move |_| {
        set_wishes.make_mut().remove(index);
    };

    let increment = move |_| set_wishes.make_mut()[index] = (skill, amount + 1);

    let decrement = move |_| set_wishes.make_mut()[index] = (skill, amount - 1);

    let skill = skill.translate(*locale);

    cx.render(rsx! {
        div { class: "control",
            button { class: "button is-danger", onclick: remove_skill,
                span { class: "icon is-small",
                    i { class: "fa-solid fa-trash" }
                }
            }
        }
        div { class: "control",
            input { class: "input", size: "15", readonly: "true", value: "{skill}" }
        },
        div { class: "control",
            button { class: "button is-link", disabled: "{is_minus_disabled}", onclick: decrement,
                span { class: "icon is-small",
                    i { class: "fa-solid fa-minus" }
                }
            }
        }
        div { class: "control",
            input { class: "input", size: "1", readonly: "true", value: "{amount}" }
        },
        div { class: "control",
            button { class: "button is-success", disabled: "{is_plus_disabled}", onclick: increment,
                span { class: "icon is-small",
                    i { class: "fa-solid fa-plus" }
                }
            }
        }
    })
}

#[inline_props]
fn AddWish<'a>(
    cx: Scope,
    options: HashSet<DisplaySkill>,
    set_wishes: &'a UseState<Vector<(DisplaySkill, u8)>>,
    locale: Locale,
) -> Element {
    let (is_active, set_is_active) = use_state(&cx, || false);

    cx.render(rsx! {
        div { class: "control",
            button { class: "button is-primary", onclick: |_| set_is_active(true),
                span { class: "icon is-small",
                    i { class: "fa-solid fa-pepper-hot" }
                },
                span { [UiSymbole::AddWish.translate(*locale)] }
            }
        }
        SelectWish {
            options: options,
            set_wishes: set_wishes,
            is_active: *is_active,
            set_is_active: set_is_active,
            locale: *locale
        },
    })
}

#[inline_props]
fn SelectWish<'a>(
    cx: Scope,
    options: &'a HashSet<DisplaySkill>,
    set_wishes: &'a UseState<Vector<(DisplaySkill, u8)>>,
    is_active: bool,
    set_is_active: &'a UseState<bool>,
    locale: Locale,
) -> Element {
    // always lowercase
    let (filter, set_filter) = use_state(&cx, String::new);

    let class = if *is_active {
        "modal is-active"
    } else {
        "modal"
    };

    let on_select = |skill| {
        move |_| {
            set_wishes.make_mut().push_back((skill, 1));
            set_is_active(false)
        }
    };

    let filter_without_diacritics = remove_diacritics(filter);

    let locale = *locale;

    let mut options: Vec<DisplaySkill> = options
        .iter()
        .copied()
        .filter(|skill| {
            remove_diacritics(&skill.translate(locale).to_lowercase())
                .contains(&filter_without_diacritics)
                || skill
                    .to_english()
                    .to_lowercase()
                    .contains(&filter_without_diacritics)
        })
        .collect();

    options.sort_unstable_by(|a, b| natural_lexical_cmp(a.translate(locale), b.translate(locale)));

    let options = options.iter().map(|&skill| {
        let text = match locale {
            Locale::English => cx.render(rsx! { [skill.to_english()] }),
            _ => {
                if !filter.is_empty() && skill.to_english().to_lowercase().contains(filter) {
                    cx.render(rsx! {
                        [skill.translate(locale)]
                        span { class: "is-italic has-text-grey-light ml-1",
                            [skill.to_english()]
                        }
                    })
                } else {
                    cx.render(rsx! {
                        [skill.translate(locale)]
                    })
                }
            }
        };
        rsx! {
            a { class: "panel-block", onclick: on_select(skill),
                text
            }
        }
    });

    let placeholder = UiSymbole::Filter.translate(locale);

    cx.render(rsx! {
        div { class: "{class}",
            div { class: "modal-background", onclick: |_| set_is_active(false) }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    div { class: "modal-card-title mr-5",
                        p { class: "control has-icons-left",
                            input {
                                class: "input is-fullwidth",
                                r#type: "text",
                                placeholder: "{placeholder}",
                                oninput: |event| set_filter(event.value.to_lowercase())
                            }
                            span { class: "icon is-left",
                                i { class: "fas fa-search" }
                            }
                        }
                    },
                    button { class: "delete", onclick: |_| set_is_active(false) }
                }
                div { class: "modal-card-body",
                    options
                }
            }
        }
    })
}
