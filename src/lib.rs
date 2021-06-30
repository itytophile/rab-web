// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use std::cmp::min;

use lexical_sort::natural_lexical_cmp;
use rab_core::armor_and_skills::Skill;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let mut sorted_skills = Skill::ALL.to_vec();
    sorted_skills.lexical_sort();
    let wishes = vec![(sorted_skills[0], 1)];
    let filtered_skills = sorted_skills
        .iter()
        .filter(|s| !wishes.iter().map(|(s, _)| s).any(|s0| *s == s0))
        .copied()
        .collect();
    Model {
        is_loading: false,
        is_choosing_skill: false,
        sorted_skills,
        filtered_skills,
        wishes,
        filter: "".to_owned(),
        changing_wish_index: None,
    }
}

trait LexicalSort {
    fn lexical_sort(&mut self);
}

impl LexicalSort for Vec<Skill> {
    fn lexical_sort(&mut self) {
        self.sort_unstable_by(|a, b| natural_lexical_cmp(&format!("{:?}", a), &format!("{:?}", b)));
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    is_loading: bool,
    is_choosing_skill: bool,
    sorted_skills: Vec<Skill>,
    filtered_skills: Vec<Skill>,
    wishes: Vec<(Skill, u8)>,
    filter: String,
    changing_wish_index: Option<usize>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    ToggleLoading,
    ChangeFilter(String),
    AddWish,
    DeleteWish(IndexWish),
    ChangeWishAmount(IndexWish, AmountSkill),
    ToggleWishModal(Option<IndexWish>),
    ChangeWish(IndexWish, Skill),
    Nothing,
}

type IndexWish = usize;
type AmountSkill = u8;

// `update` describes how to handle each `Msg`.
// I'm doing some dangerous manipulations for the sake of performance and
// memory, this is pretty useless. I'll remove them when it will be horrible
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleLoading => model.is_loading ^= true,
        Msg::ChangeFilter(filter) => {
            model.filter = filter;
            model.filtered_skills = model
                .sorted_skills
                .iter()
                .copied()
                .filter(|s| {
                    !model.wishes.iter().map(|(s, _)| s).any(|s0| s == s0)
                        && format!("{:?}", s)
                            .to_lowercase()
                            .contains(&model.filter.to_lowercase())
                })
                .collect()
        }
        Msg::AddWish => {
            // sometimes a double click can pass through a disabled button
            if model.wishes.len() < Skill::ALL.len() {
                // we can't use the filtered list because
                // the user can't see the filter input text so
                // we use the whole list (ux)
                let new_skill = model
                    .sorted_skills
                    .iter()
                    .filter(|s| !model.wishes.iter().map(|(s, _)| s).any(|s0| *s == s0))
                    .take(1)
                    .last()
                    .copied()
                    .unwrap();
                // when we add a wish we have to be sure that it's removed from the
                // filtered list
                model.filtered_skills.retain(|s| s != &new_skill);
                model.wishes.push((new_skill, 1));
            }
        }
        Msg::DeleteWish(index) => {
            if model.wishes.len() > 1 {
                let (skill, _) = model.wishes.remove(index);
                if format!("{:?}", skill)
                    .to_lowercase()
                    .contains(&model.filter.to_lowercase())
                {
                    model.filtered_skills.push(skill);
                    model.filtered_skills.lexical_sort();
                }
            }
        }
        Msg::ChangeWishAmount(index, amount) => model.wishes[index].1 = amount,
        Msg::ToggleWishModal(index) => {
            model.changing_wish_index = index;
            model.is_choosing_skill ^= true
        }
        Msg::ChangeWish(index, skill) => {
            let to_delete = model
                .filtered_skills
                .iter()
                .position(|x| *x == skill)
                .expect("skill not found in filtered list");
            model.filtered_skills.swap_remove(to_delete);
            model.filtered_skills.push(model.wishes[index].0);
            model.filtered_skills.lexical_sort();

            model.wishes[index] = (skill, 1);

            // toggling modal
            model.changing_wish_index = None;
            model.is_choosing_skill ^= true
        }
        Msg::Nothing => {}
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let disabled_delete = model.wishes.len() <= 1;
    div![
        C!["container", "has-text-centered"],
        view_modal(
            model.is_choosing_skill,
            &model.filtered_skills,
            model.changing_wish_index
        ),
        view_buttons(model.is_loading, model.wishes.len() == Skill::ALL.len()),
        model
            .wishes
            .iter()
            .enumerate()
            .map(|(index, wish)| view_wish_field(*wish, disabled_delete, index))
    ]
}

fn view_buttons(is_loading: bool, add_disabled: bool) -> Node<Msg> {
    div![
        C!["field", "is-grouped", "is-grouped-centered"],
        p![
            C!["control"],
            button![
                C!["button", "is-success", IF!(is_loading => "is-loading")],
                attrs! {At::Disabled => add_disabled.as_at_value()},
                span![C!["icon"], i![C!["fas", "fa-plus"]]],
                span!["Add wish"],
                ev(Ev::Click, |_| Msg::AddWish)
            ]
        ],
        p![
            C!["control"],
            button![
                C!["button", "is-info", IF!(is_loading => "is-loading")],
                span![C!["icon"], i![C!["fas", "fa-search"]]],
                span!["Search builds"],
                ev(Ev::Click, |_| Msg::ToggleLoading)
            ]
        ],
    ]
}

fn view_wish_field(wish: (Skill, u8), disabled_delete: bool, index: usize) -> Node<Msg> {
    let (skill, amount) = wish;
    let limit = skill.get_limit();
    div![
        C!["field", "has-addons", "has-addons-centered"],
        div![
            C!["control"],
            button![
                C!["button", "is-primary"],
                span![C!["icon"], i![C!["fas", "fa-undo"]]],
                span![format!("{:?}", skill)],
                ev(Ev::Click, move |_| Msg::ToggleWishModal(Some(index)))
            ]
        ],
        div![
            C!["control"],
            input![
                C!["input"],
                attrs! {
                    At::Type => "number",
                    At::Value => amount,
                    At::Min => 1,
                    At::Max => limit
                },
                input_ev(Ev::Input, move |new_amount| Msg::ChangeWishAmount(
                    index,
                    min(new_amount.parse::<u8>().unwrap_or(amount), limit)
                ))
            ]
        ],
        div![
            C!["control"],
            button![
                C!["button", "is-danger"],
                attrs! {At::Disabled => disabled_delete.as_at_value()},
                span![C!["icon"], i![C!["fas", "fa-trash-alt"]]],
                ev(Ev::Click, move |_| Msg::DeleteWish(index))
            ]
        ],
    ]
}

fn view_filter_input() -> Node<Msg> {
    div![
        C!["field"],
        div![
            C!["control", "has-icons-left", "has-icons-right"],
            input![
                C!["input"],
                attrs! {
                    At::Type => "text",
                    At::Placeholder => "Skill filter"
                },
                input_ev(Ev::Input, Msg::ChangeFilter)
            ],
            span![
                C!["icon", "is-small", "is-left"],
                i![C!["fas", "fa-filter"]]
            ],
        ]
    ]
}

fn view_modal(is_active: bool, skills: &[Skill], changing_wish_index: Option<usize>) -> Node<Msg> {
    div![
        C!["modal", IF!(is_active => "is-active")],
        div![C!["modal-background"]],
        div![
            C!["modal-card"],
            header![
                C!["modal-card-head"],
                p![C!["modal-card-title"], view_filter_input()]
            ],
            section![
                C!["modal-card-body"],
                // performances
                if is_active {
                    div![
                        C!["buttons"],
                        skills.iter().map(|&s| {
                            button![
                                C!["button"],
                                format!("{:?}", s),
                                if let Some(index) = changing_wish_index {
                                    ev(Ev::Click, move |_| Msg::ChangeWish(index, s))
                                } else {
                                    ev(Ev::Click, |_| Msg::Nothing)
                                },
                            ]
                        })
                    ]
                } else {
                    empty![]
                }
            ],
            footer![
                C!["modal-card-foot"],
                button![
                    C!["button", "is-link"],
                    span![C!["icon"], i![C!["fas", "fa-arrow-left"]]],
                    span!["Back"],
                    ev(Ev::Click, |_| Msg::ToggleWishModal(None))
                ],
            ]
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    App::start("root", init, update, view);
}
