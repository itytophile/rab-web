#![allow(non_snake_case)]

use anyhow::Result;
use dioxus::prelude::*;
use im_rc::{HashSet, Vector};
use rab_core::{
    armor_and_skills::{Armor, Gender, Skill},
    build_search::{pre_selection_then_brute_force_search, AllArmorSlices, Build},
};
use reqwasm::http::Request;
use ron::de::from_str;
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DisplaySkill(Skill);

impl Display for DisplaySkill {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let DisplaySkill(skill) = self;
        f.write_fmt(format_args!("{skill:?}"))
    }
}

impl Deref for DisplaySkill {
    type Target = Skill;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const BASE_URL: &str =
    "https://raw.githubusercontent.com/itytophile/monster-hunter-rise-armors/main/";

async fn fetch_armors(name: &str) -> Result<Vec<Armor>> {
    from_str(
        &Request::get(&format!("{BASE_URL}{name}.ron"))
            .send()
            .await?
            .text()
            .await?,
    )
    .map_err(|err| err.into())
}

struct AllArmors {
    helmets: Vec<Armor>,
    chests: Vec<Armor>,
    arms: Vec<Armor>,
    waists: Vec<Armor>,
    legs: Vec<Armor>,
}

impl AllArmors {
    fn as_slice(&self) -> AllArmorSlices {
        AllArmorSlices {
            arms: &self.arms,
            chests: &self.chests,
            helmets: &self.helmets,
            legs: &self.legs,
            talismans: &[],
            waists: &self.waists,
        }
    }
}

pub fn app(cx: Scope) -> Element {
    let (wishes, set_wishes) = use_state(&cx, Vector::<(DisplaySkill, u8)>::new);
    let all_skills: HashSet<DisplaySkill> = Skill::ALL.iter().copied().map(DisplaySkill).collect();
    let available_skills: HashSet<DisplaySkill> =
        all_skills.difference(wishes.iter().map(|(skill, _)| *skill).collect());
    let (all_armors, set_all_armors) = use_state(&cx, || Option::<AllArmors>::None);
    let (builds, set_builds) = use_state(&cx, Vec::<Build>::new);

    let rows = wishes.iter().enumerate().map(|(index, (skill, amount))| {
        rsx! {
            div { class: "field has-addons",
                WishRow {
                    set_wishes: set_wishes,
                    index: index,
                    skill: *skill,
                    amount: *amount
                }
            }
        }
    });

    use_future(&cx, || {
        let set = set_all_armors.to_owned();
        async move {
            set(Some(AllArmors {
                helmets: fetch_armors("helmets").await.unwrap(),
                chests: fetch_armors("chests").await.unwrap(),
                arms: fetch_armors("arms").await.unwrap(),
                waists: fetch_armors("waists").await.unwrap(),
                legs: fetch_armors("legs").await.unwrap(),
            }));
        }
    });

    let class_search_button = if all_armors.is_some() {
        "button is-info"
    } else {
        "button is-info is-loading"
    };

    let search_is_disabled = wishes.is_empty();

    let search_builds = move |_| {
        if let Some(all_armors) = all_armors {
            set_builds(pre_selection_then_brute_force_search(
                wishes
                    .iter()
                    .map(|(skill, amount)| (skill.0, *amount))
                    .collect::<Vec<(Skill, u8)>>()
                    .as_slice(),
                all_armors.as_slice(),
                Gender::Female,
                [0, 0, 0],
            ));
        }
    };

    let build_views = builds.iter().map(|build| {
        rsx! {BuildView {
            b: build
        }}
    });

    cx.render(rsx!(section { class: "section",
        div { class: "container",
            div { class: "columns",
                div { class: "column",
                    div { class: "field is-grouped",
                        AddWish {
                            options: available_skills,
                            set_wishes: set_wishes
                        }
                        div { class: "control",
                            button { class: "{class_search_button}", disabled: "{search_is_disabled}", onclick: search_builds,
                                span { class: "icon is-small",
                                    i { class: "fa-solid fa-magnifying-glass" }
                                }
                                span { "Search builds" }
                            }
                        }
                    }
                    rows
                }
                div { class: "column",
                    build_views
                }
            }
        }
    }))
}

fn armor_to_string(armor: Option<&(Armor, [Option<Skill>; 3])>) -> String {
    if let Some((armor, _)) = armor {
        armor.name.clone()
    } else {
        "Free".to_owned()
    }
}

#[inline_props] // can't use build a parameter name
fn BuildView<'a>(cx: Scope, b: &'a Build) -> Element {
    cx.render(rsx!(article { class: "panel is-primary",
        p { class: "panel-heading" }
        a { class: "panel-block",
            [armor_to_string(b.helmet.as_ref())]
        }
        a { class: "panel-block",
            [armor_to_string(b.chest.as_ref())]
        }
        a { class: "panel-block",
            [armor_to_string(b.arm.as_ref())]
        }
        a { class: "panel-block",
            [armor_to_string(b.waist.as_ref())]
        }
        a { class: "panel-block",
            [armor_to_string(b.leg.as_ref())]
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
) -> Element {
    let (index, skill, amount) = (*index, *skill, *amount);

    let is_minus_disabled = amount == 1;
    let is_plus_disabled = amount == skill.get_limit();

    let remove_skill = move |_| {
        set_wishes.make_mut().remove(index);
    };

    let increment = move |_| set_wishes.make_mut()[index] = (skill, amount + 1);

    let decrement = move |_| set_wishes.make_mut()[index] = (skill, amount - 1);

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
) -> Element {
    let (is_active, set_is_active) = use_state(&cx, || false);

    cx.render(rsx! {
        div { class: "control",
            button { class: "button is-primary", onclick: |_| set_is_active(true),
                span { class: "icon is-small",
                    i { class: "fa-solid fa-pepper-hot" }
                },
                span { "Add wish" }
            }
        }
        SelectWish {
            options: options,
            set_wishes: set_wishes,
            is_active: *is_active,
            set_is_active: set_is_active
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
) -> Element {
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

    let options = options.iter().map(|&skill| {
        rsx! {
            div { class: "panel-block", onclick: on_select(skill),
                "{skill}"
            }
        }
    });

    cx.render(rsx! {
        div { class: "{class}",
            div { class: "modal-background", onclick: |_| set_is_active(false) }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    div { class: "modal-card-title mr-5",
                        p { class: "control has-icons-left",
                            input { class: "input is-fullwidth", r#type: "text", placeholder: "Search" }
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

fn main() {
    dioxus::web::launch(app);
}
